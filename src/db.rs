use rusqlite::{Connection, Result, params};
use std::path::PathBuf;

#[derive(Clone)]
pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(config: &crate::config::Config) -> anyhow::Result<Self> {

    }
}
