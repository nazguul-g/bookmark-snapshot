use std::{collections::HashMap, fmt::Display, path::PathBuf};
use serde::{Serialize, Deserialize};
#[derive(Debug, PartialEq, Eq, Clone,Serialize,Deserialize)]
pub struct Browser {
    pub name: SupportedBrowsers,
    pub userdata_path: HashMap<SupportedOSs, String>,
    pub store_type: BookmarkStoreType,
    pub bookmark_path: Option<PathBuf>,
}
impl Browser {
    pub fn new(browser_name: SupportedBrowsers) -> Self {
        match browser_name {
            SupportedBrowsers::Brave => Browser {
                name: browser_name,
                userdata_path: SupportedBrowsers::Brave.default_path(),
                store_type: BookmarkStoreType::JSON,
                bookmark_path: None,
            },
            SupportedBrowsers::Chrome => Browser {
                name: browser_name,
                userdata_path: SupportedBrowsers::Chrome.default_path(),
                store_type: BookmarkStoreType::JSON,
                bookmark_path: None,
            },
            SupportedBrowsers::Firefox => Browser {
                name: browser_name,
                userdata_path: SupportedBrowsers::Firefox.default_path(),
                store_type: BookmarkStoreType::SQLite,
                bookmark_path: None,
            },
        }
    }
}
#[derive(Debug, PartialEq, Eq, Clone,Serialize,Deserialize)]
pub enum SupportedBrowsers {
    Brave,
    Chrome,
    Firefox,
}
impl SupportedBrowsers {
    pub fn all() -> Vec<Browser> {
        let brave = Browser::new(SupportedBrowsers::Brave);
        let firefox = Browser::new(SupportedBrowsers::Firefox);
        let chrome = Browser::new(SupportedBrowsers::Chrome);

        vec![brave, firefox, chrome]
    }
    fn default_path(&self) -> HashMap<SupportedOSs, String> {
        match &self {
            SupportedBrowsers::Chrome => {
                let mut map: HashMap<SupportedOSs, String> = HashMap::new();
                map.insert(SupportedOSs::Linux, ".config/google-chrome/".to_string());
                map.insert(SupportedOSs::Windows, r#"AppData\Local\"#.to_string());
                map
            }
            SupportedBrowsers::Brave => {
                let mut map: HashMap<SupportedOSs, String> = HashMap::new();
                map.insert(
                    SupportedOSs::Linux,
                    ".config/BraveSoftware/Brave-Browser/".to_string(),
                );
                map.insert(
                    SupportedOSs::Windows,
                    r#"\AppData\BraveSoftware\Brave-Browser\User Data\"#.to_string(),
                );
                map
            }
            SupportedBrowsers::Firefox => {
                let mut map: HashMap<SupportedOSs, String> = HashMap::new();
                map.insert(SupportedOSs::Linux, ".config/mozilla/firefox/".to_string());
                map.insert(
                    SupportedOSs::Windows,
                    r#"\AppData\Roaming\Mozilla\Firefox\Profiles\"#.to_string(),
                );
                map
            }
        }
    }
}
impl Display for SupportedBrowsers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Brave => write!(f, "brave"),
            Self::Chrome => write!(f, "chrome"),
            Self::Firefox => write!(f, "firefox"),
        }
    }
}
#[derive(Debug, Hash, PartialEq, Eq, Clone,Serialize,Deserialize)]
pub enum BookmarkStoreType {
    JSON,
    SQLite,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone,Serialize,Deserialize)]
pub enum SupportedOSs {
    Windows,
    Linux,
}
#[derive(Debug,Clone,Serialize,Deserialize)]
pub enum Routine {
    Day,
    Week,
    Month,
}

#[derive(Debug,Clone,Serialize,Deserialize )]
pub struct CliOptions {
    pub github: Option<String>,
    pub browsers: Vec<Browser>,
    pub routine: Option<Routine>,
    pub save_path: Option<String>,
    pub routine_count: u32,
    pub supported_os: Option<SupportedOSs>,
}
impl CliOptions {
    pub fn new() -> Self {
        // all options are off
        CliOptions {
            github: None,
            // support them all in if user didn't specify
            browsers: SupportedBrowsers::all(),
            routine: None,
            save_path: None,
            routine_count: 1,
            supported_os: None,
        }
    }
}
