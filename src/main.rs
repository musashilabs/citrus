use citrus::types::{PARTIAL_EXTENSIONS, classify};
use notify::{
    Event,
    EventKind::Create,
    EventKind::Modify,
    RecursiveMode, Result, Watcher,
    event::{CreateKind::File, ModifyKind, RenameMode},
};
use std::path::Path;
use std::sync::mpsc;

fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel::<Result<Event>>();

    let mut watcher = notify::recommended_watcher(tx)?;

    watcher.watch(&Path::new("."), RecursiveMode::NonRecursive)?;

    for res in rx {
        match res {
            Ok(event) => {
                if event.kind == Create(File)
                    || event.kind == Modify(ModifyKind::Name(RenameMode::Any))
                {
                    let path = event.paths.last().unwrap();

                    let filename = path.file_name().unwrap().to_string_lossy().to_string();
                    let extension = path.extension().and_then(|e| e.to_str());

                    if PARTIAL_EXTENSIONS.contains(&extension.unwrap_or_default()) {
                        continue;
                    } else {
                        // If I am here than that means there is some file which is created which has
                        // valid extension not necessarily the one on which I can act .. so I can classify it here

                        let category = classify(extension);
                        println!("{filename} -> {category:?}");
                    }
                }
            }
            Err(e) => eprintln!("Failed with error:  {}", e),
        }
    }

    Ok(())
}
