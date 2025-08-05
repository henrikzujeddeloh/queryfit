use std::vec;

use crate::models;
use anyhow::Context;
use rusqlite::{Connection, params};

#[derive(Debug)]
pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(config: &crate::config::Config) -> anyhow::Result<Self> {
        let db_path = config.get_data_path().join("database.db");

        let connection =
            Connection::open(&db_path).context("Failed to open/create database connetion")?;

        Ok(Self { conn: connection })
    }

    pub fn init_database(&self) -> anyhow::Result<()> {
        self.init_files_table()?;
        self.init_activities_table()?;

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
            .context("Failed to create metadata table")?;

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
}
