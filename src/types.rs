use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub struct Browser {
    pub name: SupportedBrowsers,
    pub bookmark_path: HashMap<SupportedOS, String>,
    pub store_type: BookmarkStoreType,
}
impl Browser {
    pub fn new(browser_name: SupportedBrowsers) -> Self {
        match browser_name {
            SupportedBrowsers::Brave => Browser {
                name: browser_name,
                bookmark_path: SupportedBrowsers::Brave.default_path(),
                store_type: BookmarkStoreType::JSON,
            },
            SupportedBrowsers::Chrome => Browser {
                name: browser_name,
                bookmark_path: SupportedBrowsers::Chrome.default_path(),
                store_type: BookmarkStoreType::JSON,
            },
            SupportedBrowsers::Firefox => Browser {
                name: browser_name,
                bookmark_path: SupportedBrowsers::Firefox.default_path(),
                store_type: BookmarkStoreType::SQLite,
            },
        }
    }
}
#[derive(Debug, PartialEq, Eq)]
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
    fn default_path(&self) -> HashMap<SupportedOS, String> {
        match &self {
            SupportedBrowsers::Chrome => {
                let mut map: HashMap<SupportedOS, String> = HashMap::new();
                map.insert(SupportedOS::Linux, ".config/google-chrome/".to_string());
                map.insert(SupportedOS::Windows, r#"AppData\Local\"#.to_string());
                map
            }
            SupportedBrowsers::Brave => {
                let mut map: HashMap<SupportedOS, String> = HashMap::new();
                map.insert(
                    SupportedOS::Linux,
                    ".config/BraveSoftware/Brave-Browser/".to_string(),
                );
                map.insert(
                    SupportedOS::Windows,
                    r#"\AppData\BraveSoftware\Brave-Browser\User Data\"#.to_string(),
                );
                map
            }
            SupportedBrowsers::Firefox => {
                let mut map: HashMap<SupportedOS, String> = HashMap::new();
                map.insert(SupportedOS::Linux, ".config/mozilla/firefox/".to_string());
                map.insert(
                    SupportedOS::Windows,
                    r#"\AppData\Roaming\Mozilla\Firefox\Profiles\"#.to_string(),
                );
                map
            }
        }
    }
}
#[derive(Debug, Hash, PartialEq, Eq)]
pub enum BookmarkStoreType {
    JSON,
    SQLite,
}
#[derive(Debug, Hash, PartialEq, Eq)]
pub enum SupportedOS {
    Windows,
    Linux,
}
#[derive(Debug)]
pub enum Routine {
    Day,
    Week,
    Month,
}

#[derive(Debug)]
pub struct CliOptions {
    pub github: Option<String>,
    pub browsers: Vec<Browser>,
    pub routine: Option<Routine>,
    pub save_path: Option<String>,
    pub routine_count: u32,
    pub supported_os: Option<SupportedOS>,
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
