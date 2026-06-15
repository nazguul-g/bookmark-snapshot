// Handle writing and reading from files
// read and write config files
// making directories , and browser related stuff
// find browsers , and related paths
// find saved bookmarks paths
// supported engines
//      chromuim
//      gecko
// supported browsers are
//      brave
//      chrome
//      tor
//      firefox

use std::{io, process::exit};

use glob::glob;

use crate::types::{Browser, COMMON_USERDATA_LOCATIONS, Options, SupportedBrowsers, SupportedOSs};

// config file strategy
// new config :
//  create new dir holds the name of software
//  create the config needed
// updating the config :
//  search for the config in the assigned path
//  add new backups subdir
//  name the add unix timestamp as file format .bak.unixtimestamp
//  back to the main dir and overwrite the file
// fallbacks :
//  existing directory :
//      ask user directly for overwrite

// Search browsers
// Option 1
// use preset of paths, and search
// Option 2
// use find command to find specifics
// Criticizing
// Option two seems not ideal. Because how do we identify each browser alone,
// we could say just find by "name of binary", but binary path and path of target data aren't the same.
//
// Option one seems like more static, does not dynamic searches for paths
//
// Final decision : we use both
//
// use a set of Known Config folders for each browser , like for brave they usually name the config folder by "BraveSoftware"
// , we also set a list of Config location to make the search faster.
//
// Each browser have its unique structure
// the actual bookmark path differs from browser to browser, gecko or chromium have nothing to do with that.
//

// checks if user specified browsers
// return
pub fn find_browsers(selected_options: &Options) -> io::Result<Vec<Browser>> {
    let all_browsers = SupportedBrowsers::all();
    let target_browsers: &[SupportedBrowsers] = if selected_options.browsers.is_empty() {
        // if user didn't specify any , we try to find them all
        &all_browsers
    } else {
        &selected_options.browsers
    };
    let mut browsers = Vec::new();
    for target in target_browsers {
        let browser = Browser::new(target);
        browsers.push(browser);
    }

    // now we have browser structure
    for browser in browsers {
        find_linux(browser);
    }
    Ok(vec![])
}
fn find_linux(browser: Browser) {
    let default_paths = COMMON_USERDATA_LOCATIONS
        .get(&SupportedOSs::Linux)
        .unwrap()
        .clone();

    let home_path = if let Ok(home) = std::env::var("HOME") {
        home
    } else {
        panic!("home dir not found")
    };
    // glob find logic
    for path in default_paths {
        //let pattern = format!("{}{}**/{}", home_path, path, browser.data_folder_id);
        let pattern = format!("{}/**/{}",home_path, browser.data_folder_id);
        println!("PATTERN : {pattern}");
        for entry in glob(&pattern).expect("failed to read the glob pattern") {
            match entry {
                Ok(path) => println!("{:?}", path),
                Err(e) => continue,
            }
        }
    }
}
