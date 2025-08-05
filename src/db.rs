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

        Self::init_database(&connection)?;

        Ok(Self { conn: connection })
    }

    fn init_database(conn: &Connection) -> anyhow::Result<()> {
        conn.execute(
            " CREATE TABLE IF NOT EXISTS metadata (
        key TEXT PRIMARY KEY,
        value TEXT NOT NULL
        )",
            params![],
        )
        .context("Failed to create metadata table")?;

        conn.execute(
            "INSERT OR REPLACE INTO metadata (key, value) VALUES (?, ?)",
            params!["version", "v0.1.0"],
        )
        .context("Failed to set database version")?;

        Ok(())
    }

    pub fn connection(&self) -> &Connection {
        &self.conn
    }
}
