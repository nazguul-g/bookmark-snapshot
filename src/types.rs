use std::{collections::HashMap, fmt::Display, sync::LazyLock};

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

// CLI RELATED TYPES
#[derive(Debug)]
pub struct Options {
    pub browsers: Vec<SupportedBrowsers>,
    pub github: Option<String>,
    pub output_dir: Option<String>,
    pub routine: Option<Routines>,
    pub os: Option<SupportedOSs>,
}
impl Options {
    pub fn new() -> Self {
        Options {
            browsers: Vec::new(),
            github: None,
            output_dir: None,
            routine: None,
            os: None,
        }
    }
    fn add_path(&mut self, browser: SupportedBrowsers, path: &str) {}
}

#[derive(Debug)]
pub enum Routines {
    Week(u32),
    Month(u32),
    Day(u32),
}

// BROWSER RELATED TYPES
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SupportedOSs {
    Windows,
    Linux,
}
impl Display for SupportedOSs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self == &Self::Linux {
            write!(f, "linux")
        } else {
            write!(f, "windows")
        }
    }
}
// these are search keywords, for identifying data folder for each browser
pub static BROWSER_DATA_FOLDER_NAME: LazyLock<HashMap<String, String>> = LazyLock::new(|| {
    let mut browser_map: HashMap<String, String> = HashMap::new();
    browser_map.insert(
        "brave".to_string(),
        "BraveSoftware/Brave-Browser".to_string(),
    );
    browser_map.insert("chrome".to_string(), "Google/Chrome".to_string());
    browser_map.insert("firefox".to_string(), "Mozilla/Firefox".to_string());
    browser_map.insert("tor".to_string(), "TorBrowser/Data/Browser".to_string());
    browser_map
});
pub static COMMON_USERDATA_LOCATIONS: LazyLock<HashMap<SupportedOSs, Vec<&str>>> =
    LazyLock::new(|| {
        let mut map: HashMap<SupportedOSs, Vec<&str>> = HashMap::new();
        map.insert(SupportedOSs::Linux, vec!["/.config/", "/.local/share/"]);
        map.insert(SupportedOSs::Windows, vec!["%AppData%", "%LocalAppData%"]);
        map
    });
#[derive(Debug, PartialEq)]
pub enum SupportedBrowsers {
    Brave,
    Chrome,
    Tor,
    FireFox,
}
impl Display for SupportedBrowsers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SupportedBrowsers::Brave => write!(f, "brave"),
            SupportedBrowsers::Chrome => write!(f, "chrome"),
            SupportedBrowsers::FireFox => write!(f, "firefox"),
            SupportedBrowsers::Tor => write!(f, "tor"),
        }
    }
}
impl SupportedBrowsers {
    pub fn all() -> Vec<Self> {
        vec![Self::Brave, Self::Chrome, Self::Tor, Self::FireFox]
    }
}
pub enum BookMarkStoreType {
    JSON,
    SQLite,
}
pub struct Browser {
    pub store_type: BookMarkStoreType,
    pub data_folder_id: String,
}
impl Browser {
    pub fn new(browser: &SupportedBrowsers) -> Self {
        let data_folders = BROWSER_DATA_FOLDER_NAME.clone();
        let browser_name = browser.to_string();

        match browser_name.as_str() {
            "brave" => Browser {
                data_folder_id: data_folders.get("brave").unwrap().clone(),
                store_type: BookMarkStoreType::JSON,
            },
            "chrome" => Browser {
                data_folder_id: data_folders.get("chrome").unwrap().clone(),
                store_type: BookMarkStoreType::JSON,
            },
            "tor" => Browser {
                data_folder_id: data_folders.get("tor").unwrap().clone(),
                store_type: BookMarkStoreType::SQLite,
            },
            "firefox" => Browser {
                data_folder_id: data_folders.get("firefox").unwrap().clone(),
                store_type: BookMarkStoreType::SQLite,
            },
            _ => unreachable!(
                "unsupported browser ,{}. this statement is impossigble to reach",
                browser_name
            ),
        }
    }
}

pub struct Config {
    pub options: Options,
}
