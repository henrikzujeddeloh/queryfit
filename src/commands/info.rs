use crate::VERSION;
use crate::config::Config;
use crate::db::Database;
use anyhow::Context;
use clap::Args;
use rusqlite::params;
use walkdir::WalkDir;

#[derive(Debug, Args)]
pub struct InfoArgs {}

impl InfoArgs {
    pub fn run(&self, config: &Config, db: &Database) -> anyhow::Result<()> {
        println!("{:<25}: {}", "App version", VERSION);
        println!("{:<25}: {:?}", "Database version", Self::get_version(db)?);
        if !db.get_db_validitiy() {
            println!("{:<25}: {}", "Database Status", "INVALID");
            println!("Please run 'queryfit database recreate'. No data will be lost.");
        }
        println!("{:<25}: {:?}", "Data location", config.get_data_path());
        println!(
            "{:<25}: {}",
            "Imported .fit files",
            Self::get_num_files_in_db(db)?
        );
        println!(
            "{:<25}: {}",
            "Total .fit files",
            Self::get_num_fit_files(config)?
        );
        println!(
            "{:<25}: {}",
            "Total .fit files size",
            Self::format_file_size(Self::get_all_data_size(config)?)
        );
        println!(
            "{:<25}: {}",
            "Database size",
            Self::format_file_size(Self::get_db_size(db)?)
        );

        Ok(())
    }

    fn get_version(db: &Database) -> anyhow::Result<String> {
        let conn = db.connection();

        conn.query_row(
            "SELECT value FROM metadata WHERE key = 'version'",
            params![],
            |row| row.get(0),
        )
        .context("Failed to get version from database")
    }

    fn get_num_files_in_db(db: &Database) -> anyhow::Result<i64> {
        let count: i64 =
            db.connection()
                .query_row("SELECT COUNT(*) FROM files", params![], |row| row.get(0))?;
        Ok(count)
    }

    fn get_num_fit_files(config: &Config) -> anyhow::Result<i64> {
        let count: usize = WalkDir::new(config.get_data_path())
            .into_iter()
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                entry
                    .path()
                    .extension()
                    .map_or(false, |ext| ext.eq_ignore_ascii_case("fit"))
            })
            .count();

        Ok(count as i64)
    }

    fn get_all_data_size(config: &Config) -> anyhow::Result<u64> {
        let size: u64 = WalkDir::new(config.get_data_path())
            .into_iter()
            .filter_map(|entry| entry.ok())
            .filter_map(|entry| {
                let path = entry.path();
                if path.is_file()
                    && path
                        .extension()
                        .map_or(false, |ext| ext.eq_ignore_ascii_case("fit"))
                {
                    path.metadata().ok().map(|metadata| metadata.len())
                } else {
                    None
                }
            })
            .sum();
        Ok(size)
    }

    fn get_db_size(db: &Database) -> anyhow::Result<u64> {
        let page_count: i64 = db
            .connection()
            .query_row("PRAGMA page_count", params![], |row| row.get(0))?;
        let page_size: i64 = db
            .connection()
            .query_row("PRAGMA page_size", params![], |row| row.get(0))?;
        let size = page_count as u64 * page_size as u64;
        Ok(size)
    }

    fn format_file_size(bytes: u64) -> String {
        const KB: f64 = 1_000.0;
        const MB: f64 = KB * 1_000.0;
        const GB: f64 = MB * 1_000.0;

        let (size, unit) = if bytes >= (GB as u64) {
            (bytes as f64 / GB, "GB")
        } else if bytes >= (MB as u64) {
            (bytes as f64 / MB, "MB")
        } else if bytes >= (KB as u64) {
            (bytes as f64 / KB, "KB")
        } else {
            (bytes as f64, "bytes")
        };

        format!("{:.2} {}", size, unit)
    }
}
