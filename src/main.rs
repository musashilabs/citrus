use std::path::Path;
use notify::{Event, RecursiveMode, Result, Watcher};
use std::sync::mpsc;
use notify::event::CreateKind::{File, Folder};
use notify::EventKind::Create;

fn main() -> Result<()> {
    let (tx,rx) = mpsc::channel::<Result<Event>>();

    let mut watcher = notify::recommended_watcher(tx)?;

    watcher.watch(&Path::new("."), RecursiveMode::NonRecursive)?;

    for res in rx {
        match res {
            Ok(event) =>{
                if event.kind == Create(File) || event.kind == Create(Folder){
                    let path = event.paths.last().unwrap();
                    println!("Created: {:?}", path.file_name().unwrap().to_string_lossy());
                }
            }
            Err(e) => eprintln!("Failed with error: {}", e)
        }
    }

    Ok(())
}