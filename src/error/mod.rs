#[derive(thiserror::Error, Debug)]
pub enum ConfigError {
    #[error("could not determine config directory")]
    NoConfigDir,
    #[error("failed to read config file: {0}")]
    Read(#[from] std::io::Error),
    #[error("failed to parse config file: {0}")]
    Parse(#[from] toml::de::Error),
}
