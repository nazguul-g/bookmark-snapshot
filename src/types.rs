use std::{collections::HashMap, sync::LazyLock};

// the project related types
// bookmark tree type
pub enum BookmarkTree {
    Folder(Folder),
    Link(String),
}
pub struct Folder {
    pub name: String,
    pub folder: Vec<BookmarkTree>,
}

#[derive(Debug)]
pub struct Options {
    pub browsers: Vec<Browsers>,
    pub github: Option<String>,
    pub output_dir: Option<String>,
    pub routine: Option<Routines>,
}
impl Options {
    pub fn new() -> Self {
        Options {
            browsers: Vec::new(),
            github: None,
            output_dir: None,
            routine: None,
        }
    }
    fn add_path(&mut self, browser: Browsers, path: &str) {}
}

#[derive(Debug)]
pub enum Routines {
    Week(u32),
    Month(u32),
    Day(u32),
}

pub static BROWSER_DATA_FOLDER_NAME: LazyLock<HashMap<String, String>> = LazyLock::new(|| {
    let mut browser_map: HashMap<String, String> = HashMap::new();
    browser_map.insert(
        "brave".to_string(),
        "BraveSoftware/Brave-Browser".to_string(),
    );
    browser_map.insert("chrome".to_string(), "Google/Chrome".to_string());
    browser_map.insert("firefox".to_string(), "Mozilla/Firefox".to_string());
    browser_map.insert(
        "tor".to_string(),
        "Tor Browser/Browser/TorBrowser/Data/Browser".to_string(),
    );
    browser_map
});
#[derive(Debug)]
pub enum Browsers {
    Brave(String),
    Chrome(String),
    Tor(String),
    FireFox(String),
}
pub enum BookMarkStoreType {
    JSON,
    SQLite,
}
pub struct Browser {
    pub data_folder_name: String,
    pub store_type: BookMarkStoreType,
    pub data_folder_location: String,
}
impl Browser {
    pub fn new() {}
}
