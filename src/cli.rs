use crate::commands;
use crate::config::Config;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "queryfit")]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(name = "info")]
    Info(commands::InfoArgs),
}

impl Cli {
    pub fn run(self) -> anyhow::Result<()> {
        let config = Config::load()?;

        match self.commands {
            Commands::Info(cmd) => cmd.run(&config),
        }
    }
}
