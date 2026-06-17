use std::{
    path::{Path, PathBuf},
};

use crate::types::{Browser, CliOptions, SupportedOS};

pub fn check_path(path: &str) -> bool {
    let path = Path::new(path);
    if path.exists() {
        return true;
    }
    false
}

pub fn search_browsers(options: CliOptions) {}
