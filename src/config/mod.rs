use crate::error::ConfigError;
use directories::ProjectDirs;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub watch: WatchConfig,
    pub destinations: HashMap<String, String>,
    pub extensions: HashMap<String, Vec<String>>,
    pub partial: PartialConfig,
}

#[derive(Deserialize, Debug)]
pub struct PartialConfig {
    pub extensions: Vec<String>
}

#[derive(Deserialize, Debug)]
pub struct WatchConfig {
    pub path: String,
}

const DEFAULT_CONFIG: &str = include_str!("../../default_config.toml");

pub fn config_path() -> Result<PathBuf, ConfigError> {
    let proj_dirs = ProjectDirs::from("dev", "rohit", "citrus").ok_or(ConfigError::NoConfigDir)?;
    Ok(proj_dirs.config_dir().join("config.toml"))
}

pub fn load_or_create_config() -> Result<Config, ConfigError> {
    let path = config_path()?;

    if !path.exists() {
        let parent = path.parent().ok_or(ConfigError::NoConfigDir)?;
        fs::create_dir_all(parent)?;
        fs::write(&path, DEFAULT_CONFIG)?;
    }

    let contents = fs::read_to_string(&path)?;
    let config = toml::from_str(&contents)?;
    Ok(config)
}
