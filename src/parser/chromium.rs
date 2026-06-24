// Handle and parse bookmarks of chromium based browsers
#[derive(Serialize, Deserialize)]
pub struct JSONBookmarkSchema {}
use std::{
    collections::HashMap,
    error::Error,
    fs::{File, OpenOptions, create_dir_all},
    io::{self, BufReader, BufWriter, ErrorKind},
    path::{self, Path},
    time::{SystemTime, UNIX_EPOCH},
};

use serde::{Deserialize, Serialize};

use crate::{
    io::{
        browsers::{check_path, get_home_directory, get_input},
        config::{get_config_linux, get_config_windows},
    },
    parser::types::ChromiumBookmarks,
    types::{BookmarkStoreType, Browser, CliOptions, SupportedOSs},
};
pub fn snapshot() -> Result<(), Box<dyn Error>> {
    let os = std::env::consts::OS;

    let config = match os {
        "linux" => get_config_linux()?,
        "windows" => get_config_windows()?,
        _ => {
            return Err(Box::new(io::Error::new(
                ErrorKind::Unsupported,
                "operating system not supported",
            )));
        }
    };
    let save_path = config.save_path.unwrap();
    // exclusive to linux . windows default fallback not covered
    for b in &config.browsers {
        match b.store_type {
            BookmarkStoreType::JSON => json_parser(b, &save_path)?,
            BookmarkStoreType::SQLite => sqlite_parser(b, &save_path)?,
        }
    }
    Ok(())
}
fn json_parser(browser: &Browser, save_directory: &str) -> Result<(), Box<dyn Error>> {
    let browser = browser.clone();

    let bookmark_path = browser.bookmark_path.clone().unwrap();
    let reader = read_file(&bookmark_path.to_str().unwrap())?;
    let bookmarks_parsed: ChromiumBookmarks = serde_json::from_reader(reader)?;
    let path = Path::new(&save_directory)
        .join("Bookmarks Snapshots")
        .join(browser.name.to_string());
    let directory = create_dir_all(&path)?;
    let writer = write_file(&path.join(&generate_name(&browser)))?;
    serde_json::to_writer_pretty(writer, &bookmarks_parsed)?;
    println!(
        "{} bookmark snapshot saved at: '{}'",
        browser.name,
        path.display()
    );

    Ok(())
}
fn sqlite_parser(browser: &Browser, save_directory: &str) -> Result<(), Box<dyn Error>> {
    Ok(())
}

fn generate_name(browser: &Browser) -> String {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time went backwards")
        .as_secs();
    let file_name = format!("{}_snapshot{}.json", browser.name, timestamp);
    file_name
}
fn read_file(path: &str) -> Result<BufReader<File>, Box<dyn Error>> {
    let file = OpenOptions::new().read(true).open(path)?;
    let reader = BufReader::new(file);
    Ok(reader)
}
// assuming file not exists
fn write_file(path: &Path) -> Result<BufWriter<File>, Box<dyn Error>> {
    let file = OpenOptions::new().write(true).create(true).open(path)?;
    let writer = BufWriter::new(file);
    Ok(writer)
}
 