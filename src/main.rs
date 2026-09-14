use citrus::cli::{Cli, Commands, handle_start, handle_stop, print_log_head, print_log_tail};
use citrus::config;
use clap_builder::Parser;
use notify::Result;

fn main() -> Result<()> {
    let cli = Cli::parse();
    let log_path = config::log_path().expect("Could not determine the log path");
    let pid_path = config::pid_path().expect("Could not determine the pid path");

    match cli.command {
        Some(Commands::Log { head, tail }) => {
            if let Some(n) = head {
                print_log_head(&log_path, n);
            } else if let Some(n) = tail {
                print_log_tail(&log_path, n);
            } else {
                eprintln!("specify --head <N> or --tail <N>");
            }
            return Ok(());
        }

        Some(Commands::Start) => handle_start(&pid_path, &log_path),

        Some(Commands::Stop) => handle_stop(&pid_path),

        None => {
            println!("citrus — watches a folder and auto-sorts new files by type\n");
            println!("USAGE:");
            println!("  citrus start           Start watching (daemonizes)");
            println!("  citrus stop            Stop the running daemon");
            println!("  citrus log --tail N    Show last N log lines");
            println!("  citrus log --head N    Show first N log lines");
            println!("\nRun `citrus --help` for full details.");
        }
    }

    Ok(())
}
