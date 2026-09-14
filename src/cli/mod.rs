use crate::config::{pid_is_alive, read_existing_pid};
use crate::types::{classify, destination_for};
use crate::{build_extension_map, config, expand_tilde, log_line, move_file};
use clap::{Parser, Subcommand};
use daemonize::Daemonize;
use nix::sys::signal::{Signal, kill};
use nix::unistd::Pid;
use notify::EventKind::{Create, Modify};
use notify::event::CreateKind::File;
use notify::event::{ModifyKind, RenameMode};
use notify::{Event, RecursiveMode, Watcher};
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::sync::mpsc;

#[derive(Parser)]
#[command(name = "dsorter", about = "Watches a folder and auto-sorts new files by type")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Start watching and sorting (daemonizes)
    Start,
    /// Stop the running daemon
    Stop,
    /// View recent log entries
    Log {
        /// Show the first N log lines
        #[arg(long)]
        head: Option<usize>,
        /// Show the last N log lines
        #[arg(long)]
        tail: Option<usize>,
    },
}

pub fn print_log_head(log_path: &Path, n: usize) {
    let contents = fs::read_to_string(log_path).unwrap_or_default();
    for line in contents.lines().take(n) {
        println!("{line}");
    }
}

pub fn print_log_tail(log_path: &Path, n: usize) {
    let contents = fs::read_to_string(log_path).unwrap_or_default();
    let lines: Vec<&str> = contents.lines().collect();
    let start = lines.len().saturating_sub(n);
    for line in &lines[start..] {
        println!("{line}");
    }
}

pub fn handle_start(pid_path: &Path, log_path: &Path) {
    if let Some(pid) = read_existing_pid(pid_path) {
        if pid_is_alive(pid) {
            eprintln!("dsorter is already running (pid {pid})");
            std::process::exit(1);
        } else {
            let _ = fs::remove_file(pid_path); // stale pidfile, clean it up
        }
    }

    let stdout = fs::File::create(log_path).unwrap();
    let stderr = stdout.try_clone().unwrap();

    let daemonize = Daemonize::new().pid_file(pid_path).stdout(stdout).stderr(stderr);

    if let Err(e) = daemonize.start() {
        eprintln!("failed to daemonize: {e}");
        std::process::exit(1);
    }

    let config = match config::load_or_create_config() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("config error: {e}");
            std::process::exit(1);
        }
    };

    println!("{config:#?}");
    let ext_map = build_extension_map(&config);
    let partial: HashSet<String> =
        config.partial.extensions.iter().map(|s| s.to_lowercase()).collect();

    let watch_path = expand_tilde(&config.watch.path);

    let (tx, rx) = mpsc::channel::<notify::Result<Event>>();
    let mut watcher = notify::recommended_watcher(tx).unwrap();
    watcher.watch(&watch_path, RecursiveMode::NonRecursive).unwrap();

    for res in rx {
        match res {
            Ok(event) => {
                if event.kind == Create(File)
                    || event.kind == Modify(ModifyKind::Name(RenameMode::Any))
                {
                    let path = event.paths.last().unwrap();

                    //prevents spamming for partial downloaded files
                    if !path.exists() {
                        continue;
                    }
                    let filename = path.file_name().unwrap().to_string_lossy().to_string();
                    let extension = path.extension().and_then(|e| e.to_str());

                    if let Some(ext) = extension
                        && partial.contains(&ext.to_lowercase())
                    {
                        continue;
                    }

                    let category = classify(extension, &ext_map);
                    println!("{:?} -> {category:?}", path.file_name().unwrap());

                    if let Some(dest_dir) = destination_for(category, &config) {
                        match move_file(path, &dest_dir) {
                            Ok(()) => log_line(
                                log_path,
                                &format!("moved {filename} -> {}", dest_dir.display()),
                            ),
                            Err(e) => {
                                eprintln!("failed to move {:?}: {e}", path.file_name().unwrap())
                            }
                        }
                    }
                }
            }
            Err(e) => eprintln!("Failed with error:  {}", e),
        }
    }
}

pub fn handle_stop(pid_path: &Path) {
    match read_existing_pid(pid_path) {
        Some(pid) if pid_is_alive(pid) => {
            kill(Pid::from_raw(pid), Signal::SIGTERM).expect("failed to send SIGTERM");
            let _ = fs::remove_file(pid_path);
            println!("stopped dsorter (pid {pid})");
        }
        _ => {
            eprintln!("dsorter is not running");
        }
    }
}
