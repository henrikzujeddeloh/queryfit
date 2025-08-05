use std::fs;
use std::path::PathBuf;
use serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
pub struct Config {
    data: PathBuf,
}

impl Config {
    fn load(path: &str) -> anyhow::Result<Self> {
        let file_contents: String = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&file_contents)?;
        Ok(config)
    }
}
