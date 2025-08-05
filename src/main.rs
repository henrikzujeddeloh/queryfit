mod cli;
mod commands;
mod config;
mod db;
mod models;

use clap::Parser;
use cli::Cli;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    cli.run()
}
