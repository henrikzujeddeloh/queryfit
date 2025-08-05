use crate::config::Config;
use clap::Args;

#[derive(Debug, Args)]
pub struct InfoCommand {

}

impl InfoCommand {
    pub fn run(&self, config: &Config, db: &Database) -> anyhow::Result<()>  {
        Ok(())
    }
}
