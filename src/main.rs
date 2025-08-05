mod cli;
mod config;
mod commands;

use cli::Cli;
use clap::Parser;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    cli.run()
}
