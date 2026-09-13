use crate::config::Config;
use crate::expand_tilde;
use std::collections::HashMap;
use std::path::PathBuf;

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

    pub(crate) fn to_key(self) -> &'static str {
        match self {
            Self::Image => "image",
            Self::Doc => "doc",
            Self::Archive => "archive",
            Self::Video => "video",
            Self::Audio => "audio",
            Self::Other => "other",
        }
    }
}

pub fn classify(ext: Option<&str>, ext_map: &HashMap<String, Category>) -> Category {
    match ext {
        Some(ext) => ext_map.get(&ext.to_lowercase()).copied().unwrap_or(Category::Other),

        None => Category::Other,
    }
}

pub fn destination_for(category: Category, config: &Config) -> Option<PathBuf> {
    config.destinations.get(category.to_key()).map(|s| expand_tilde(s))
}
