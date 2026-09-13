use crate::config::Config;
use crate::types::Category;
use std::collections::HashMap;
use std::path::PathBuf;

pub fn build_extension_map(config: &Config) -> HashMap<String, Category> {
    let mut map = HashMap::new();
    for (key, extensions) in &config.extensions {
        let category = Category::from_key(key);
        for ext in extensions {
            map.insert(ext.to_lowercase(), category);
        }
    }
    map
}

pub fn expand_tilde(path: &str) -> PathBuf {
    if let Some(rest) = path.strip_prefix("~/") {
        let home = dirs::home_dir().expect("could not determine home directory");
        home.join(rest)
    } else {
        PathBuf::from(path)
    }
}
