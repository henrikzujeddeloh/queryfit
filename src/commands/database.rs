use crate::config::Config;
use crate::db::Database;
use crate::models::{Activity, Device, File};
use anyhow::anyhow;
use clap::{Args, Subcommand};
use fitparser::Value;
use fitparser::de::{DecodeOption, FitObject, FitStreamProcessor};
use fitparser::profile::MesgNum;
use indicatif::ProgressBar;
use rusqlite::{Connection, params};
use std::collections::HashSet;
use std::path::Path;
use std::process;
use std::time::{Duration, Instant};

#[derive(Debug, Args)]
pub struct DatabaseArgs {
    #[command(subcommand)]
    pub actions: Actions,
}

#[derive(Debug, Subcommand)]
pub enum Actions {
    #[command(name = "import")]
    #[command(about = "import new .fit files into database")]
    Import,

    #[command(name = "recreate")]
    #[command(about = "recreate database from all .fit files")]
    Recreate,
}

impl DatabaseArgs {
    pub fn run(&self, config: &Config, db: &Database) -> anyhow::Result<()> {
        match &self.actions {
            Actions::Import => {
                Self::run_import(config, db)?;
            }
            Actions::Recreate => {
                Self::run_recreate(&self, config, db)?;
            }
        }
        Ok(())
    }

    fn run_import(config: &Config, db: &Database) -> anyhow::Result<()> {
        // do not allow import if database is invalid
        if !db.get_db_validitiy() {
            println!(
                "The database version does not match the app version.\nPlease run 'queryfit database recreate'.\nNo data will be lost."
            );
            process::exit(0);
        }

        let data_path = config.get_data_path();

        let files: Vec<_> = walkdir::WalkDir::new(data_path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map_or(false, |ext| ext == "fit"))
            .collect();

        let pb = ProgressBar::new(files.len() as u64);

        println!("Adding .fit file data to database...");
        let tx = db.connection().unchecked_transaction()?;
        let profile_import = std::env::var_os("QUERYFIT_PROFILE_IMPORT").is_some();
        let import_started = Instant::now();
        let mut total_parse_time = Duration::ZERO;
        let mut total_insert_time = Duration::ZERO;
        // TODO: process multiple files in parallel?
        for item in files {
            let fit_file_path = item.path();
            let file = File::new(Self::get_filename(&fit_file_path)?);

            // move to next file if it exists in database
            if Self::check_file_imported(&file, &tx)? {
                pb.inc(1);
                continue;
            }

            let parse_started = Instant::now();
            let activity_data = Self::read_activity(&fit_file_path)?;
            total_parse_time += parse_started.elapsed();

            let insert_started = Instant::now();
            Self::add_activity(activity_data, &tx)?;
            Self::add_filename(&file, &tx)?;
            total_insert_time += insert_started.elapsed();

            pb.inc(1);
        }
        tx.commit()?;
        pb.finish_and_clear();
        if profile_import {
            println!(
                "import timing: total={:.2?}, fit_decode={:.2?}, db_insert={:.2?}",
                import_started.elapsed(),
                total_parse_time,
                total_insert_time
            );
        }
        println!("done.");

        Ok(())
    }

    pub fn run_recreate(&self, config: &Config, db: &Database) -> anyhow::Result<()> {
        db.reset()?;
        db.init_database()?;
        db.set_db_valid();
        Self::run_import(config, db)?;
        Ok(())
    }

    fn read_activity(path: &Path) -> anyhow::Result<(Vec<Activity>, Vec<Device>)> {
        let opts: HashSet<DecodeOption> = HashSet::from([
            DecodeOption::SkipHeaderCrcValidation,
            DecodeOption::SkipDataCrcValidation,
            DecodeOption::DropUnknownFields,
            DecodeOption::DropUnknownMessages,
        ]);

        let buffer = std::fs::read(path)?;
        let mut input = buffer.as_slice();
        let mut processor = FitStreamProcessor::new();

        opts.iter().for_each(|option| processor.add_option(*option));

        let mut sessions: Vec<Activity> = Vec::new();
        let mut curr_session = Activity::new();

        let mut devices: Vec<Device> = Vec::new();
        let mut curr_device = Device::new();

        while !input.is_empty() {
            let (remaining, object) = processor.deserialize_next(input)?;
            let record = match object {
                FitObject::Crc(..) => {
                    processor.reset();
                    input = remaining;
                    continue;
                }
                FitObject::DataMessage(message)
                    if Self::should_decode_message(message.global_message_number()) =>
                {
                    processor.decode_message(message)?
                }
                _ => {
                    input = remaining;
                    continue;
                }
            };

            match record.kind() {
                MesgNum::Session => {
                    if !curr_session.is_empty() {
                        sessions.push(curr_session);
                        curr_session = Activity::new();
                    }
                    for field in record.fields() {
                        match field.name() {
                            "sport" => curr_session.sport = field.value().to_string(),
                            "total_timer_time" => {
                                curr_session.duration = field.clone().into_value().try_into()?
                            }
                            "total_distance" => {
                                let distance: f64 = field.clone().into_value().try_into()?;
                                curr_session.distance =
                                    // store 0 distance as None/NULL
                                    if distance > 0.0 { Some(distance) } else { None };
                                // curr_session.distance = Some(field.clone().into_value().try_into()?)
                            }
                            "avg_heart_rate" => {
                                curr_session.avg_hr = field.clone().into_value().try_into()?
                            }
                            "start_time" => match field.clone().into_value() {
                                Value::Timestamp(local_dt) => {
                                    // timestamps in .fit file are in local time
                                    curr_session.timestamp = local_dt
                                }
                                _ => eprintln!("Unexpected timestamp type"),
                            },
                            "total_calories" => {
                                curr_session.calories = field.clone().into_value().try_into()?
                            }
                            "total_ascent" => {
                                let total_ascent: f64 = field.clone().into_value().try_into()?;
                                curr_session.elevation = if total_ascent > 0.0 {
                                    Some(total_ascent)
                                } else {
                                    None
                                };
                            }
                            "avg_power" => {
                                let avg_power: f64 = field.clone().into_value().try_into()?;
                                curr_session.avg_power = if avg_power > 0.0 {
                                    Some(avg_power)
                                } else {
                                    None
                                };
                            }
                            "workout_rpe" => {
                                let workout_rpe: f64 = field.clone().into_value().try_into()?;
                                curr_session.rpe = if workout_rpe > 0.0 {
                                    Some(workout_rpe/10.0)
                                } else {
                                    None
                                };
                            }
                            _ => {}
                        }
                    }
                }
                MesgNum::DeviceInfo => {
                    if !curr_device.is_empty() {
                        devices.push(curr_device);
                        curr_device = Device::new();
                    }
                    for field in record.fields() {
                        // println!("{} ---- {}", field.name(), field);
                        match field.name() {
                            // TODO: better handle these special device definitions
                            "product" => match field.value().to_string().as_str() {
                                "1052" => curr_device.product = "sram_power".to_string(),
                                "1037" => curr_device.product = "sram_shifting".to_string(),
                                _ => curr_device.product = field.value().to_string(),
                            },
                            "garmin_product" => {
                                curr_device.product = field.value().to_string();
                            }
                            "timestamp" => match field.clone().into_value() {
                                Value::Timestamp(local_dt) => curr_device.timestamp = local_dt,
                                _ => eprintln!("Unexpected timestamp type"),
                            },
                            "battery_voltage" => {
                                curr_device.battery = Some(field.clone().into_value().try_into()?)
                            }
                            "battery_status" => {
                                curr_device.battery_status = Some(field.value().to_string())
                            }
                            "descriptor" | "product_name" => {
                                let name = field.value().to_string();
                                if !name.is_empty() {
                                    curr_device.product = name;
                                }
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }

            input = remaining;
        }
        if !curr_session.is_empty() {
            sessions.push(curr_session);
        }
        if !curr_device.is_empty() {
            devices.push(curr_device);
        }

        Ok((sessions, devices))
    }

    fn add_activity(
        activity_data: (Vec<Activity>, Vec<Device>),
        conn: &Connection,
    ) -> anyhow::Result<()> {
        let (sessions, devices) = activity_data;

        for session in sessions {
            conn.execute(
                "INSERT INTO activities (sport, timestamp, duration, distance, calories, avg_hr, elevation, avg_power, rpe, rpe_est) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
                params![session.sport, session.timestamp.to_rfc3339(), session.duration, session.distance, session.calories, session.avg_hr, session.elevation, session.avg_power, session.rpe, session.rpe_est],
            )?;
        }

        for device in devices {
            conn.execute(
                "INSERT INTO devices (product, timestamp, battery, battery_status) VALUES (?1, ?2, ?3, ?4)",
                params![
                    device.product,
                    device.timestamp.to_rfc3339(),
                    device.battery,
                    device.battery_status,
                ],
            )?;
        }
        Ok(())
    }

    fn should_decode_message(global_message_number: u16) -> bool {
        matches!(
            MesgNum::from(global_message_number),
            MesgNum::Session | MesgNum::DeviceInfo | MesgNum::FieldDescription
        )
    }

    fn get_filename(path: &Path) -> anyhow::Result<String> {
        let filename = match path.file_name().and_then(|name| name.to_str()) {
            Some(name) => Ok(name),
            None => Err(anyhow!(format!("Failed to find filename of {:?}", path))),
        }?;
        Ok(filename.to_owned())
    }

    fn check_file_imported(file: &File, conn: &Connection) -> anyhow::Result<bool> {
        let exists: bool = conn.query_row(
            "SELECT EXISTS(SELECT 1 FROM files WHERE filename = ?1)",
            params![file.filename],
            |row| row.get(0),
        )?;

        Ok(exists)
    }

    fn add_filename(file: &File, conn: &Connection) -> anyhow::Result<()> {
        conn.execute(
            "INSERT INTO files (filename) VALUES (?1)",
            params![file.filename],
        )?;

        Ok(())
    }
}
