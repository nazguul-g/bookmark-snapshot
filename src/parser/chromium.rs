// Handle and parse bookmarks of chromium based browsers
#[derive(Serialize, Deserialize)]
pub struct JSONBookmarkSchema {}
use std::{
    error::Error, fs::{File, OpenOptions, create_dir_all}, io::{self, BufReader, BufWriter}, path, time::{SystemTime, UNIX_EPOCH},
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
    let output_directory = match options.save_path {
        None => format!("{}/Documents/Bookmark-Snapshots/{}",get_home_directory(),browser.name),
        Some(path) => format!("{}/Bookmark-Snapshots/{}",path,browser.name),
    };
    create_dir_all(&output_directory)?;
    println!("{output_directory}");
    let bookmark_path =browser.bookmark_path.clone().unwrap();
    let file_path = format!("{}/{}",output_directory,generate_name(&browser));
    let reader = read_file(bookmark_path.to_str().unwrap())?;
    let bookmarks :ChromiumBookmarks= serde_json::from_reader(reader)?;
    let writer = write_file(&file_path)?;
    println!("bookmarks snapshoted, at {}", file_path);
    serde_json::to_writer_pretty(writer,&bookmarks)?;
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
fn read_file(path:&str) -> Result<BufReader<File>,Box<dyn Error>>{
    let file = OpenOptions::new().read(true).open(path)?;
    let reader = BufReader::new(file);
    Ok(reader)
}
// assuming file not exists
fn write_file(path:&str) -> Result<BufWriter<File>, Box<dyn Error>>{
    if check_path(path) {
        return Err(Box::new(io::Error::new(io::ErrorKind::AlreadyExists, "file already exists")));
    }
    let file = OpenOptions::new().write(true).create(true).open(path)?;
    let writer = BufWriter::new(file);
    Ok(writer)
}
