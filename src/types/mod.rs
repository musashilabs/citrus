use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub enum Category {
    Image,
    Doc,
    Archive,
    Video,
    Audio,
    Other,
}

impl Category {
    pub(crate) fn from_key(key: &str) -> Category {
        match key {
            "image" => Category::Image,
            "doc" => Category::Doc,
            "archive" => Category::Archive,
            "video" => Category::Video,
            "audio" => Category::Audio,
            _ => Category::Other,
        }
    }
}

pub fn classify(ext: Option<&str>, ext_map: &HashMap<String, Category>) -> Category {
    match ext {
        Some(ext) => ext_map
            .get(&ext.to_lowercase())
            .copied()
            .unwrap_or(Category::Other),

        None => Category::Other,
    }
}
