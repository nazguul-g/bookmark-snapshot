// Handle and parse bookmarks of chromium based browsers
#[derive(Serialize, Deserialize)]
pub struct JSONBookmarkSchema {}
use std::{
    error::Error,
    fs::OpenOptions,
    io::{BufReader, BufWriter},
    time::{SystemTime, UNIX_EPOCH},
};

use serde::{Deserialize, Serialize};

use crate::{
    io::{
        browsers::{check_path, get_home_directory, get_input},
        config::get_config,
    },
    parser::types::ChromiumBookmarks,
    types::Browser,
};
pub fn chromium_parser(browser: &Browser) -> Result<(), Box<dyn Error>> {
    let options = get_config()?;
    // supposing save path already defined;save path is directory path
    let save_path = if let Some(path) = options.save_path {
        path
    } else {
        format!(
            "{}/Documents/{}",
            get_home_directory(),
            generate_name(&browser)
        )
    };
    let bookmark_path = browser.bookmark_path.clone().unwrap();
    let file = OpenOptions::new().read(true).open(bookmark_path)?;
    let reader = BufReader::new(file);
    let json: ChromiumBookmarks = serde_json::from_reader(reader)?;

    let save_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(save_path)?;
    let writer = BufWriter::new(save_file);
    serde_json::to_writer_pretty(writer, &json)?;

    Ok(())
}
fn generate_name(browser: &Browser) -> String {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time went backwards")
        .as_secs();
    let file_name = format!("{}_snapshot_{}.json", browser.name, timestamp);
    file_name
}
