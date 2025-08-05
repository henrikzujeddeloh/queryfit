use crate::config::Config;
use clap::Args;

#[derive(Debug, Args)]
pub struct InfoArgs {

}

impl InfoArgs {
    pub fn run(&self, config: &Config) -> anyhow::Result<()>  {
        Ok(())
    }
}
