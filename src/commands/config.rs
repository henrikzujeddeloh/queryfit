use crate::config::Config;
use clap::{Args, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Args)]
pub struct ConfigArgs {
    #[command(subcommand)]
    pub actions: Actions,
}

#[derive(Debug, Subcommand)]
pub enum Actions {
    #[command(name = "path")]
    #[command(about = "print the queryfit config file path")]
    Path,

    #[command(name = "show")]
    #[command(about = "print the current queryfit config")]
    Show,

    #[command(name = "get")]
    #[command(about = "get a queryfit config value")]
    Get(GetArgs),

    #[command(name = "set")]
    #[command(about = "set a queryfit config value")]
    Set(SetArgs),
}

#[derive(Debug, Args)]
pub struct GetArgs {
    #[command(subcommand)]
    pub key: ConfigKey,
}

#[derive(Debug, Args)]
pub struct SetArgs {
    #[command(subcommand)]
    pub key: SetConfigKey,
}

#[derive(Debug, Subcommand)]
pub enum ConfigKey {
    #[command(name = "data")]
    #[command(about = "get the data directory")]
    Data,
}

#[derive(Debug, Subcommand)]
pub enum SetConfigKey {
    #[command(name = "data")]
    #[command(about = "set the data directory")]
    Data { path: PathBuf },
}

impl ConfigArgs {
    pub fn run(&self) -> anyhow::Result<()> {
        match &self.actions {
            Actions::Path => {
                println!("{}", Config::get_config_file_path().display());
            }
            Actions::Show => {
                let config = Config::load_or_create()?;
                println!("data = {:?}", config.get_data_path());
            }
            Actions::Get(args) => {
                let config = Config::load_or_create()?;
                match args.key {
                    ConfigKey::Data => println!("{}", config.get_data_path().display()),
                }
            }
            Actions::Set(args) => {
                let mut config = Config::load_or_create()?;
                match &args.key {
                    SetConfigKey::Data { path } => {
                        config.set_data_path(path.to_owned());
                        config.save()?;
                        println!("Set data to {}", config.get_data_path().display());
                    }
                }
            }
        }

        Ok(())
    }
}
