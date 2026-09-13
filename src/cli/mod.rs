use clap::{Parser, Subcommand};
use std::fs;
use std::path::Path;

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Start,
    Log {
        #[arg(long)]
        head: Option<usize>,
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
