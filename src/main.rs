use citrus::cli::{Cli, Commands, print_log_head, print_log_tail};
use citrus::types::{classify, destination_for};
use citrus::{build_extension_map, config, expand_tilde, log_line, move_file};
use clap_builder::Parser;
use notify::{
    Event,
    EventKind::Create,
    EventKind::Modify,
    RecursiveMode, Result, Watcher,
    event::{CreateKind::File, ModifyKind, RenameMode},
};
use std::collections::HashSet;
use std::sync::mpsc;

fn main() -> Result<()> {
    let cli = Cli::parse();
    let log_path = config::log_path().expect("Could not determine the log path");

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
        Some(Commands::Start) | None => {
            // existing config-load + watch-loop code goes here,
            // using log_line(&log_path, ...) instead of println!/eprintln!
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

            let (tx, rx) = mpsc::channel::<Result<Event>>();
            let mut watcher = notify::recommended_watcher(tx)?;
            watcher.watch(&watch_path, RecursiveMode::NonRecursive)?;

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
                                        &log_path,
                                        &format!("moved {filename} -> {}", dest_dir.display()),
                                    ),
                                    Err(e) => {
                                        eprintln!(
                                            "failed to move {:?}: {e}",
                                            path.file_name().unwrap()
                                        )
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => eprintln!("Failed with error:  {}", e),
                }
            }
        }
    }

    Ok(())
}
