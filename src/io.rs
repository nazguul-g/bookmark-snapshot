use std::{
    collections::HashMap,
    env::home_dir,
    f64::consts::E,
    io,
    path::{Path, PathBuf},
    process::exit,
};

use colored::Colorize;
use dialoguer::{Input, Select};
use glob::glob;

use crate::types::{BookmarkStoreType, Browser, CliOptions, SupportedBrowsers, SupportedOSs};

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
pub fn search_browsers(options: &mut CliOptions) {
    // check using the default paths
    // prompt user if not found any bookmarks
    let mut browsers = options.browsers.clone();

    for b in browsers.iter_mut() {
        let bookmark_path = match options.supported_os {
            Some(SupportedOSs::Linux) => glob_search_bookmarks_linux(b),
            Some(SupportedOSs::Windows) => todo!(),
            None => unreachable!(),
        };
        match bookmark_path {
            Some(path) => {
                b.bookmark_path = Some(path)
            }
            None => {
                todo!()
            }
        }
    }
    options.browsers  = browsers;
    println!("{:?}",options.browsers);
}

// For chromium based browsers, the pattern is "Bookmark" without any extension.
// For gecko based browsers, they often use single file for storing all user data,
// the file is sqlite DB names "places.sqlite", the bookmark table is name "moz_bookmarks" which is a reference to another table named "moz_places".

// The job now is to look for this two patterns using given browser userdata path
fn glob_search_bookmarks_linux(browser: &mut Browser) -> Option<PathBuf> {
    let pattern = match browser.store_type {
        BookmarkStoreType::JSON => pattern_builder_linux(
            browser.userdata_path.get(&SupportedOSs::Linux).unwrap(),
            BookmarkStoreType::JSON,
        ),
        BookmarkStoreType::SQLite => pattern_builder_linux(
            browser.userdata_path.get(&SupportedOSs::Linux).unwrap(),
            BookmarkStoreType::SQLite,
        ),
    };
    match glob(&pattern) {
        Ok(paths) => {
            for entry in paths {
                match entry {
                    Ok(path) => {
                        println!("found bookmark at {}", path.display());
                        return Some(path);
                    }
                    Err(e) => {
                        eprintln!("glob search error: {e}");
                        exit(1)
                    }
                };
            }
            let user_defined_path = request_path(&browser.name);
            Some(Path::new(&user_defined_path).to_path_buf())
        }
        Err(e) => {
            eprintln!("glob search error: {e}");
            exit(1)
        }
    }
}
pub fn get_input(message: &str) -> String {
    let input: String = Input::new()
        .with_prompt(format!("{}", message.on_bright_red()))
        .interact()
        .unwrap();
    input
}
pub fn request_path(name: &SupportedBrowsers) -> String {
    let choices = vec!["yes", "quit"];
    let selection = Select::new()
        .with_prompt(format!(
            "found no bookmarks for {}. would u want to specify user data folder path?",
            name
        ))
        .items(&choices)
        .interact()
        .unwrap();
    let path = match selection {
        0 => get_input(&format!("{}", "userdata folder path: ".bright_green())),
        _ => exit(1),
    };
    if check_path(&path) {
        return path;
    } else {
        request_path(name)
    }
}
fn glob_search_bookmarks_windows(pattern: &str) {}
fn pattern_builder_linux(userdata: &str, store_type: BookmarkStoreType) -> String {
    let pattern = match store_type {
        BookmarkStoreType::JSON => {
            format!("{}/{}*/{}", get_home_directory(), userdata, "Bookmarks")
        }
        BookmarkStoreType::SQLite => {
            format!("{}/{}*/{}", get_home_directory(), userdata, "places.sqlite")
        }
    };
    println!("{pattern}");
    pattern
}
