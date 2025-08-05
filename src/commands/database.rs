use crate::config::Config;
use crate::db::Database;
use crate::models::Activity;
use crate::models::File;
use anyhow::anyhow;
use clap::{Args, Subcommand};
use fitparser::{
    de::{DecodeOption, from_reader_with_options},
    profile::MesgNum,
};
use rusqlite::params;
use std::collections::HashSet;
use std::path::Path;

#[derive(Debug, Args)]
pub struct DatabaseArgs {
    #[command(subcommand)]
    pub actions: Actions,
}

#[derive(Debug, Subcommand)]
pub enum Actions {
    #[command(name = "import")]
    Import,

    #[command(name = "recreate")]
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
        let data_path = config.get_data_path();

        for item in walkdir::WalkDir::new(data_path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map_or(false, |ext| ext == "fit"))
        {
            let fit_file_path = item.path();

            // move to next file if it exists in database
            if Self::check_file_imported(fit_file_path, db)? {
                continue;
            }

            Self::add_activity(fit_file_path, db)?;

            Self::add_filename(fit_file_path, db)?
        }
        Ok(())
    }

    fn run_recreate(&self, config: &Config, db: &Database) -> anyhow::Result<()> {
        println!("recreate");
        Ok(())
    }

    fn add_activity(path: &Path, db: &Database) -> anyhow::Result<()> {
        let mut activity = Activity::new();

        let mut file = std::fs::File::open(path)?;

        let opts: HashSet<DecodeOption> = HashSet::from([
            DecodeOption::SkipHeaderCrcValidation,
            DecodeOption::SkipDataCrcValidation,
            DecodeOption::DropUnknownFields,
            DecodeOption::DropUnknownMessages,
        ]);
        let records = from_reader_with_options(&mut file, &opts)?;

        for record in records {
            match record.kind() {
                MesgNum::Sport => {
                    for field in record.fields() {
                        match field.name() {
                            "sport" => activity.sport_type = field.value().to_string(),
                            _ => {}
                        }
                    }
                }
                MesgNum::Activity => {
                    for field in record.fields() {
                        match field.name() {
                            // TODO: figure out how to do this without cloning (.try_into() not
                            // implemented for f64 and borrowed fields?)
                            "total_timer_time" => {
                                activity.duration = field.clone().into_value().try_into()?
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
        db.connection().execute("INSERT INTO activities (sport_type, duration) VALUES (?1, ?2)", params![activity.sport_type, activity.duration])?;
        Ok(())
    }

    fn check_file_imported(path: &Path, db: &Database) -> anyhow::Result<bool> {
        let filename = match path.file_name().and_then(|name| name.to_str()) {
            Some(name) => Ok(name),
            None => Err(anyhow!(format!("Failed to find filename of {:?}", path))),
        }?;

        let exists: bool = db.connection().query_row(
            "SELECT EXISTS(SELECT 1 FROM files WHERE filename = ?1)",
            params![filename],
            |row| row.get(0),
        )?;

        Ok(exists)
    }

    fn add_filename(path: &Path, db: &Database) -> anyhow::Result<()> {
        let filename = match path.file_name().and_then(|name| name.to_str()) {
            Some(name) => Ok(name),
            None => Err(anyhow!(format!("Failed to find filename of {:?}", path))),
        }?;
        db.connection().execute(
            "INSERT INTO files (filename) VALUES (?1)",
            params![filename],
        )?;

        Ok(())
    }
}
