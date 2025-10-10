use crate::commands;
use crate::config::Config;
use crate::db::Database;
use clap::{Parser, Subcommand};
use std::process;

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

    #[command(name = "summary")]
    #[command(about = "display summary over specified time period")]
    Summary(commands::SummaryArgs),

    #[command(name = "devices")]
    #[command(about = "get information on devices")]
    Devices(commands::DevicesArgs),

    #[command(name = "calculate")]
    #[command(about = "calculate something from workout data")]
    Calculate(commands::CalculateArgs),
}

impl Cli {
    pub fn run(self) -> anyhow::Result<()> {
        let config = Config::load()?;

        let db = Database::new(&config)?;

        if db.initialized()? {
            if !db.correct_version()? {
                db.set_db_invalid();
                // if database version is not correct, only allow database commands
                match &self.commands {
                    Commands::Info(cmd) => cmd.run(&config, &db)?,
                    Commands::Database(cmd) => cmd.run(&config, &db)?,
                    _ => println!(
                        "The database version does not match the app version.\n Please run'queryfit database recreate'.\nNo data will be lost."
                    ),
                }
                process::exit(0);
            }
        } else {
            db.init_database()?;
        }

        match self.commands {
            Commands::Info(cmd) => cmd.run(&config, &db),
            Commands::Database(cmd) => cmd.run(&config, &db),
            Commands::Summary(cmd) => cmd.run(&config, &db),
            Commands::Devices(cmd) => cmd.run(&config,&db),
            Commands::Calculate(cmd) => cmd.run(&config,&db),
        }
    }
}
