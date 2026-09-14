use crate::error::ConfigError;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Serialize, Default, Deserialize)]
pub struct Status {
    pub(crate) watching: String,
    pub(crate) pid: i32,
    pub(crate) started_at: u64,
    pub(crate) last_action: Option<String>,
    pub(crate) files_sorted: u64,
}

pub fn status_path() -> Result<PathBuf, ConfigError> {
    let proj_dirs = ProjectDirs::from("com", "rohit", "dsorter").ok_or(ConfigError::NoConfigDir)?;
    let data_dir = proj_dirs.data_dir();
    fs::create_dir_all(data_dir)?;
    Ok(data_dir.join("dsorter_status.json"))
}

pub fn write_status(status_path: &Path, status: &Status) {
    if let Ok(json) = serde_json::to_string_pretty(status) {
        let _ = fs::write(status_path, json);
    }
}
