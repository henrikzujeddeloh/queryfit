mod cli;
mod commands;
mod config;
mod db;
mod models;

use clap::Parser;
use cli::Cli;

pub static VERSION: &str = "v0.3.0";

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    cli.run()
}
