use std::path::Path;
use notify::{Event, RecursiveMode, Result, Watcher};
use std::sync::mpsc;
use notify::event::CreateKind::{File, Folder};
use notify::EventKind::Create;
use citrus::types::{FileMapper, FileType};

fn main() -> Result<()> {

    let mut file_mapper = FileMapper::new();
    let (tx,rx) = mpsc::channel::<Result<Event>>();

    let mut watcher = notify::recommended_watcher(tx)?;

    watcher.watch(&Path::new("."), RecursiveMode::NonRecursive)?;

    for res in rx {
        match res {
            Ok(event) =>{
                if event.kind == Create(File) || event.kind == Create(Folder){
                    let path = event.paths.last().unwrap();

                    dbg!("Created: {:?}", path.file_name().unwrap().to_string_lossy());

                    let filename = path.file_name().unwrap().to_string_lossy().to_string();
                    match path.extension().unwrap().to_str().unwrap() {
                        ".jpg"|".png"|".gif"|".jpeg"|".heif" => {
                            file_mapper.records.push(FileType::Image(filename))
                        }
                        ".pdf"|".docx"|".txt"| "xlsx" => {
                            file_mapper.records.push(FileType::Doc(filename))
                        }
                        ".zip"|".tar"|".rar" => {
                            file_mapper.records.push(FileType::Archive(filename))
                        }
                        ".mkv"|".mp4"|"mov" => {
                            file_mapper.records.push(FileType::Video(filename))
                        }
                        ".mp3"|".wav"|".mp4a" => {
                            file_mapper.records.push(FileType::Audio(filename))
                        }
                        _ => {
                            dbg!("Unable to map the file");
                        }
                    }
                }
            }
            Err(e) => eprintln!("Failed with error:  {}", e)

        }
    }

    Ok(())
}