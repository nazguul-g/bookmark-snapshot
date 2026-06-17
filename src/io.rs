use std::{
    io,
    path::{Path, PathBuf},
    process::exit,
};

use glob::glob;

use crate::types::{BookmarkStoreType, Browser, CliOptions, SupportedBrowsers, SupportedOS};

pub fn check_path(path: &str) -> bool {
    let path = Path::new(path);
    if path.exists() {
        return true;
    }
    false
}
fn get_home_directory() -> String {
    match std::env::var("HOME") {
        Ok(home) => home,
        Err(_) => {
            eprintln!("home directory not found, quiting");
            exit(1)
        }
    }
}
pub fn search_browsers(options: &CliOptions) {
    let browsers = options.browsers.clone();
    for browser in browsers {
        match options.supported_os {
            Some(SupportedOS::Linux) => {
                let pattern = format!(
                    "{}/{}",
                    get_home_directory(),
                    browser.userdata_path.get(&SupportedOS::Linux).unwrap()
                );
                glob_search_bookmarks_linux(&pattern, browser.store_type);
            }
            Some(SupportedOS::Windows) => {
                let pattern = browser.userdata_path.get(&SupportedOS::Windows).unwrap();
                glob_search_bookmarks_windows(&pattern);
            }
            _ => (),
        }
    }
}

// For chromium based browsers, the pattern is "Bookmark" without any extension.
// For gecko based browsers, they often use single file for storing all user data,
// the file is sqlite DB names "places.sqlite", the bookmark table is name "moz_bookmarks" which is a reference to another table named "moz_places".

// The job now is to look for this two patterns using given browser userdata path
fn glob_search_bookmarks_linux(pattern: &str, store_type: BookmarkStoreType) {
    let mut pattern = pattern.to_string();
    match store_type {
        BookmarkStoreType::JSON => {
            pattern = format!("{}{}",pattern,"*/Bookmarks");
        },
        BookmarkStoreType::SQLite => {
            pattern = format!("{}{}",pattern,"*/places.sqlite");
        },
    }
    match glob(&pattern) {
            Ok(paths) => {
                for entry in paths {
                    match entry {
                        Ok(path) => println!("Found bookmarks at: {}", path.display()),
                        Err(e) => eprintln!("Error reading path: {:?}", e),
                    }
                }
            },
            Err(e) => eprintln!("Failed to compile glob pattern: {}", e),
        }
}
fn glob_search_bookmarks_windows(pattern: &str) {}
