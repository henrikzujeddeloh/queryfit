use crate::VERSION;
use anyhow::Context;
use rusqlite::{Connection, params};
use std::cell::Cell;

#[derive(Debug)]
pub struct Database {
    conn: Connection,
    valid: Cell<bool>,
}

impl Database {
    pub fn new(config: &crate::config::Config) -> anyhow::Result<Self> {
        let db_path = config.get_data_path().join("database.db");

        let connection =
            Connection::open(&db_path).context("Failed to open/create database connetion")?;

        Ok(Self {
            conn: connection,
            valid: true.into(),
        })
    }

    pub fn reset(&self) -> anyhow::Result<()> {
        let tables: Vec<String> = self
            .conn
            .prepare("SELECT name FROM sqlite_master WHERE type='table'")?
            .query_map([], |row| row.get(0))?
            .collect::<Result<Vec<String>, rusqlite::Error>>()?;

        for table in tables {
            println!("droppping {}", table);
            self.conn
                .execute(&format!("DROP TABLE IF EXISTS {}", table), [])?;
        }

        Ok(())
    }

    pub fn initialized(&self) -> anyhow::Result<bool> {
        let exists: bool = self.conn.query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table'",
            params![],
            |row| row.get(0),
        )?;

        Ok(exists)
    }

    pub fn correct_version(&self) -> anyhow::Result<bool> {
        let db_version: String = self
            .conn
            .query_row(
                "SELECT value FROM metadata WHERE key = 'version'",
                params![],
                |row| row.get(0),
            )
            .context("Could not retrieve version from metadata table")?;

        Ok(db_version == VERSION)
    }

    pub fn init_database(&self) -> anyhow::Result<()> {
        self.init_metadata_table()?;
        self.init_files_table()?;
        self.init_activities_table()?;

        Ok(())
    }

    fn init_metadata_table(&self) -> anyhow::Result<()> {
        self.conn
            .execute(
                "CREATE TABLE IF NOT EXISTS metadata (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
                )",
                params![],
            )
            .context("Failed to create metadata table")?;

        self.conn.execute(
            "INSERT OR REPLACE INTO metadata (key, value) VALUES ('version', ?1)",
            params![VERSION.to_string()],
        )?;
        Ok(())
    }

    fn init_files_table(&self) -> anyhow::Result<()> {
        self.conn
            .execute(
                "CREATE TABLE IF NOT EXISTS files (
                filename TEXT PRIMARY KEY
                )",
                params![],
            )
            .context("Failed to create files table")?;

        Ok(())
    }

    fn init_activities_table(&self) -> anyhow::Result<()> {
        self.conn
            .execute(
                "CREATE TABLE IF NOT EXISTS activities (
                id INTEGER PRIMARY KEY,
                sport_type TEXT NOT NULL,
                duration REAL
                )
                ",
                params![],
            )
            .context("Failed to create activites table")?;

        Ok(())
    }

    pub fn connection(&self) -> &Connection {
        &self.conn
    }

    pub fn set_db_invalid(&self) {
        self.valid.set(false);
    }

    pub fn set_db_valid(&self) {
        self.valid.set(true);
    }

    pub fn get_db_validitiy(&self) -> bool {
        self.valid.get()
    }
}
