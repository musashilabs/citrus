pub const PARTIAL_EXTENSIONS: [&str; 3] =["crdownload", "part", "download"];

#[derive(Debug)]
pub enum Category {
    Image,
    Doc,
    Archive,
    Video,
    Audio,
    Other,
}

pub fn classify(ext: Option<&str>) -> Category {
    match ext {
        Some(ext) => match ext.to_lowercase().as_str() {
            "jpg" | "png" | "gif" | "jpeg" | "heif" => Category::Image,
            "pdf" | "docx" | "txt" | "xlsx" => Category::Doc,
            "zip" | "tar" | "rar" => Category::Archive,
            "mkv" | "mp4" | "mov" => Category::Video,
            "mp3" | "wav" | "mp4a" => Category::Audio,
            _ => Category::Other,
        },

        None => Category::Other,
    }
}
