use std::path::Path;
use notify::{Event, RecursiveMode, Result, Watcher};
use std::sync::mpsc;


fn main()-> Result<()> {
    let (tx,rx) = mpsc::channel::<Result<Event>>();

    let mut watcher = notify::recommended_watcher(tx)?;

    watcher.watch(&Path::new("."), RecursiveMode::Recursive)?;

    for res in rx {
        match res {
            Ok(val) => println!("Found this: {:?}", val),
            Err(e) => eprintln!("Failed with error: {}", e)
        }
    }

    Ok(())
}