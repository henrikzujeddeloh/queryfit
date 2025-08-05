use clap::{Parser, Subcommand};
use crate::commands;
use crate::config::Config;

#[derive(Debug, Parser)]
#[command(name="queryfit")]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(name = "info")]
    Info(commands::InfoCommand),
}

impl Cli {
    pub fn run(self) -> anyhow::Result<()> {

    let config = Config::load()?;

    match self.command {
            Commands::Info(cmd) => cmd.run(&config),
        }
    }
}
