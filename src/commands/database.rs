use crate::config::Config;
use crate::db::Database;
use crate::models::{Activity, File};
use anyhow::anyhow;
use clap::{Args, Subcommand};
use fitparser::Value;
use fitparser::de::{DecodeOption, from_reader_with_options};
use fitparser::profile::MesgNum;
use indicatif::ProgressBar;
use rusqlite::params;
use std::collections::HashSet;
use std::path::Path;
use std::process;

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
        // TODO: process multiple files in parallel?
        for item in files {
            let fit_file_path = item.path();
            let file = File::new(Self::get_filename(&fit_file_path)?);

            // move to next file if it exists in database
            if Self::check_file_imported(&file, db)? {
                pb.inc(1);
                continue;
            }

            Self::add_activity(&fit_file_path, db)?;
            Self::add_filename(&file, db)?;

            pb.inc(1);
        }
        pb.finish_and_clear();
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

    fn add_activity(path: &Path, db: &Database) -> anyhow::Result<()> {
        let mut file = std::fs::File::open(path)?;

        let opts: HashSet<DecodeOption> = HashSet::from([
            DecodeOption::SkipHeaderCrcValidation,
            DecodeOption::SkipDataCrcValidation,
            DecodeOption::DropUnknownFields,
            DecodeOption::DropUnknownMessages,
        ]);
        let records = from_reader_with_options(&mut file, &opts)?;

        let mut sessions: Vec<Activity> = Vec::new();
        let mut curr_session = Activity::new();

        for record in records {
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
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
        if !curr_session.is_empty() {
            sessions.push(curr_session);
        }
        for session in sessions {
            db.connection().execute(
                "INSERT INTO activities (sport, timestamp, duration, distance, calories) VALUES (?1, ?2, ?3, ?4, ?5)",
                params![session.sport, session.timestamp.to_rfc3339(), session.duration, session.distance, session.calories],
            )?;
        }
        Ok(())
    }

    fn get_filename(path: &Path) -> anyhow::Result<String> {
        let filename = match path.file_name().and_then(|name| name.to_str()) {
            Some(name) => Ok(name),
            None => Err(anyhow!(format!("Failed to find filename of {:?}", path))),
        }?;
        Ok(filename.to_owned())
    }

    fn check_file_imported(file: &File, db: &Database) -> anyhow::Result<bool> {
        let exists: bool = db.connection().query_row(
            "SELECT EXISTS(SELECT 1 FROM files WHERE filename = ?1)",
            params![file.filename],
            |row| row.get(0),
        )?;

        Ok(exists)
    }

    fn add_filename(file: &File, db: &Database) -> anyhow::Result<()> {
        db.connection().execute(
            "INSERT INTO files (filename) VALUES (?1)",
            params![file.filename],
        )?;

        Ok(())
    }
}
