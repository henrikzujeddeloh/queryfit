use crate::config::Config;
use crate::db::Database;
use anyhow::Context;
use clap::Args;
use rusqlite::params;

#[derive(Debug, Args)]
pub struct InfoArgs {}

impl InfoArgs {
    pub fn run(&self, config: &Config, db: &Database) -> anyhow::Result<()> {
        println!("Data location: {:?}", config.get_data_path());
        println!("Database version: {:?}", Self::get_version(db));
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
}
