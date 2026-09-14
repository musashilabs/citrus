use crate::error::ConfigError;
use directories::ProjectDirs;
use nix::sys::signal::kill;
use nix::unistd::Pid;
use std::fs;
use std::path::{Path, PathBuf};

pub fn pid_path() -> Result<PathBuf, ConfigError> {
    let proj_dirs = ProjectDirs::from("com", "rohit", "citrus").ok_or(ConfigError::NoConfigDir)?;
    let data_dir = proj_dirs.data_dir();
    fs::create_dir_all(data_dir)?;
    Ok(data_dir.join("citrus.pid"))
}

pub fn pid_is_alive(pid: i32) -> bool {
    kill(Pid::from_raw(pid), None).is_ok()
}

pub fn read_existing_pid(pid_path: &Path) -> Option<i32> {
    fs::read_to_string(pid_path).ok()?.trim().parse().ok()
}
