use crate::config::Config;
use crate::db::Database;
use clap::{Args, Subcommand};

#[derive(Debug, Args)]
pub struct DatabaseArgs {
    #[command(subcommand)]
    pub actions: Actions
}

#[derive(Debug, Subcommand)]
pub enum Actions {
    #[command(name = "import")]
    Import {
        #[arg(long)]
        test: Option<String>,
    },
    #[command(name = "recreate")]
    Recreate,
}


impl DatabaseArgs {
    pub fn run(&self, config: &Config, db: &Database) -> anyhow::Result<()> {
        match &self.actions {
            Actions::Import { test } => {
                Self::run_import(test, config, db)?;
            },
            Actions::Recreate => {
                Self::run_recreate(&self, config, db)?;
            }
        }
        Ok(())
    }

    fn run_import(test: &Option<String>, config: &Config, db: &Database) -> anyhow::Result<()> {
        println!("import");
        println!("test: {:?}", test);
        Ok(())
    }

    fn run_recreate(&self, config: &Config, db: &Database) -> anyhow::Result<()> {
        println!("recreate");
        Ok(())
    }
}
