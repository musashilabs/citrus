use citrus::types::classify;
use notify::{Event, EventKind::Create, RecursiveMode, Result, Watcher, event::CreateKind::File};
use std::path::Path;
use std::sync::mpsc;

fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel::<Result<Event>>();

    let mut watcher = notify::recommended_watcher(tx)?;

    watcher.watch(&Path::new("."), RecursiveMode::NonRecursive)?;

    for res in rx {
        match res {
            Ok(event) => {
                if event.kind == Create(File) {
                    let path = event.paths.last().unwrap();

                    let filename = path.file_name().unwrap().to_string_lossy().to_string();
                    let extension = path.extension().and_then(|e| e.to_str());

                    let category = classify(extension);
                    println!("{filename} -> {category:?}");
                }
            }
            Err(e) => eprintln!("Failed with error:  {}", e),
        }
    }

    Ok(())
}
