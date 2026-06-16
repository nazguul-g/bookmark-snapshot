#[derive(Debug)]
pub struct Browser {
    pub name: String,
    pub bookmark_path: String,
    pub store_type: BookmarkStoreType,
}
#[derive(Debug)]
pub enum SupportedBrowsers {
    Brave,
    Chrome,
    Firefox,
}
impl SupportedBrowsers {
    pub fn all() -> Vec<Browser> {
        let brave = Browser {
            name: "brave".to_string(),
            bookmark_path: String::new(),
            store_type: BookmarkStoreType::JSON,
        };
        let firefox = Browser {
            name: "firefox".to_string(),
            bookmark_path: String::new(),
            store_type: BookmarkStoreType::SQLite,
        };
        let chrome = Browser {
            name: "chrome".to_string(),
            bookmark_path: String::new(),
            store_type: BookmarkStoreType::JSON,
        };

        vec![brave, firefox, chrome]
    }
    pub fn get_browser(self) -> Browser {
        match &self {
            SupportedBrowsers::Brave => Browser {
                name: "brave".to_string(),
                bookmark_path: String::new(),
                store_type: BookmarkStoreType::JSON,
            },
            SupportedBrowsers::Chrome => Browser {
                name: "chrome".to_string(),
                bookmark_path: String::new(),
                store_type: BookmarkStoreType::JSON,
            },
            SupportedBrowsers::Firefox => Browser {
                name: "firefox".to_string(),
                bookmark_path: String::new(),
                store_type: BookmarkStoreType::SQLite,
            },
        }
    }
}
#[derive(Debug)]
pub enum BookmarkStoreType {
    JSON,
    SQLite,
}
#[derive(Debug)]
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
