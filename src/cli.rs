use crate::commands;
use crate::config::Config;
use crate::db::Database;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "queryfit", version)]
#[command(about = "A CLI tool for analyzing .fit data")]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(name = "info")]
    #[command(about = "information about .fit files and database")]
    Info(commands::InfoArgs),

    #[command(name = "database")]
    #[command(about = "importing into or recreating the database")]
    Database(commands::DatabaseArgs),
}

impl Cli {
    pub fn run(self) -> anyhow::Result<()> {
        let config = Config::load()?;

        let db = Database::new(&config)?;

        match self.commands {
            Commands::Info(cmd) => cmd.run(&config, &db),
            Commands::Database(cmd) => cmd.run(&config, &db),
        }
    }
}
