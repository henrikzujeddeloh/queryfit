use anyhow::Context;
use dirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use toml;

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Config {
    data: PathBuf,
}

static DEFAULT_CONFIG: &str = r#"
            data = '/path/to/data'
        "#;

impl Config {
    // get queryfit config directory
    fn get_config_dir() -> PathBuf {
        dirs::config_dir()
            .expect("Could not find config directory")
            .join("queryfit")
    }

    // get full path to config.toml file
    fn get_config_file_path() -> PathBuf {
        Self::get_config_dir().join("config.toml")
    }

    // ensure config directory exists
    fn ensure_config_dir() -> anyhow::Result<()> {
        let config_dir = Self::get_config_dir();
        fs::create_dir_all(&config_dir).context("Could not create config directory")?;
        Ok(())
    }

    // load Config from config.toml
    pub fn load() -> anyhow::Result<Self> {
        Self::ensure_config_dir()?;

        let config_path = Self::get_config_file_path();

        if !config_path.exists() {
            Self::create_default_config();
            std::process::exit(0);
        }

        let file_contents =
            fs::read_to_string(&config_path).context("Failed to read config file")?;

        let config: Config =
            toml::from_str(&file_contents).context("Failed to parse config.toml")?;

        Ok(config)
    }

    fn create_default_config() -> anyhow::Result<Self> {
        let config_path = Self::get_config_file_path();

        // define default config.toml
        let default_config: Self =
            toml::from_str(DEFAULT_CONFIG).context("Failed to parse default config raw string")?;

        let config_toml_string = toml::to_string_pretty(&default_config)
            .context("Failed to serialize default config to toml")?;

        fs::write(&config_path, config_toml_string).context("Failed to write default config")?;

        println!(
            "Created default configuration at {:?}. Please edit before continuing!",
            &config_path
        );

        Ok(default_config)
    }

    pub fn get_data_path(&self) -> &PathBuf {
        &self.data
    }
}
