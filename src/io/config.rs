use std::{
    error::Error,
    fs::{self, OpenOptions},
    io::{self, BufReader, BufWriter, ErrorKind::NotFound},
};

use schemars::Schema;
static LINUX_CONFIG_PATH: &str = ".config/BookmarkSnapshot/";
static WINDOWS_CONFIG_PATH: &str = "";
use crate::{
    io::browsers::{check_path, get_home_directory},
    types::{CliOptions, SupportedOSs},
};
fn create_dir(path: &str) -> io::Result<()> {
    fs::create_dir_all(path)
}
pub fn save_config_linux(cli_options: &CliOptions) -> Result<(), Box<dyn Error>> {
    let options = cli_options.clone();
    let config_file_name = "options_config.json";
    let os = options.supported_os.as_ref().unwrap();
    let config_path = match os {
        SupportedOSs::Linux => format!("{}/{}", get_home_directory(), LINUX_CONFIG_PATH),
        SupportedOSs::Windows => WINDOWS_CONFIG_PATH.to_string(),
    };
    if !check_path(&config_path) {
        create_dir(&config_path).unwrap();
    }
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(format!("{}{}", config_path, config_file_name))?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &options);
    Ok(())
}
pub fn save_config_windows(cli_options: &CliOptions) -> Result<(), Box<dyn Error>> {
    Ok(())
}
pub fn get_config_linux() -> Result<CliOptions, Box<dyn Error>> {
    let path = format!(
        "{}/{}options_config.json",
        get_home_directory(),
        LINUX_CONFIG_PATH
    );
    if !check_path(&path) {
        return Err(Box::new(io::Error::new(
            NotFound,
            "config directory not found",
        )));
    }
    let file = OpenOptions::new()
        .read(true)
        .truncate(false)
        .write(false)
        .open(path)?;
    let reader = BufReader::new(file);
    let options: CliOptions = serde_json::from_reader(reader)?;
    // extra validation schema validation is already done with deserializing
    match validate_schema() {
        Ok(_) => return Ok(options),
        Err(_) => {
            return Err(Box::new(io::Error::new(
                io::ErrorKind::InvalidInput,
                "config file already exists, contains different schema",
            )));
        }
    }
}
// we must validate schema is identical as well
// config might exist but what about the schema
// this decision adds more validation for config to be overwritten
//
// expected to input a schema and json , check if json follows that schema , return Ok or Err
pub fn validate_schema() -> Result<(), ()> {
    Ok(())
}
#[test]
fn test_validate_schema() {}
