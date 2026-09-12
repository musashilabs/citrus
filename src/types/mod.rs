pub enum FileType {
    Image(String),
    Doc(String),
    Archive(String),
    Video(String),
    Audio(String)
}

pub struct FileMapper {
   pub records: Vec<FileType>
}

impl FileMapper {
    pub fn new()->Self{
        Self {
            records: Vec::new()
        }
    }
}