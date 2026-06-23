// Handle parsing CLI arguments

use std::{
    fs::TryLockError::Error,
    io,
    path::{Path, PathBuf},
    process::exit,
};

use clap::{Arg, ArgMatches, Command};
use colored::Colorize;
use dialoguer::Input;

use crate::{
    io::{
        browsers::{check_path, get_input, search_browsers},
        config::save_config,
    },
    parser::chromium::chromium_parser,
    types::{Browser, CliOptions, Routine, SupportedBrowsers, SupportedOSs},
};

// Unless user specified the path, we use predefined paths for system to search.
// the path logic is broken
// decision:
//  automatically search for browsers
//  if none found we ask user to input path

/*
main command : --browser <Browsers> (not required , looks for available browsers) , subcommand : --path <PATH> (not required , looks for default paths) ,
main command : --github <REPO_URL> (not required , save local only).
main command : --outputpath <DIR_PATH> (not required, save in Documents).
main command : --routine <DAY/WEEK/MONTH> , (not required, save one time only) , subcommand: --count <INTEGER> (not required, routine each DAY/WEEK/MONTH)

future commands :
--status (check the status of the progam , return info about the options)
-- update (using this as prefix followed by the original commands , make the program update the config )
*/

// due to luck of documentation about clap derive, decided to use clap builder instead
pub fn cli() -> io::Result<()> {
    let matches = Command::new("Bookmarks snapshot")
        .about("Automaticlly save browser booksmarks")
        .version("1.0-alpha")
        .arg(
            Arg::new("browser")
                .long("browser")
                .short('b')
                .value_name("BROWSER")
                .help("Supported browser are : brave, firefox, chrome")
                .required(false)
                .value_parser(["brave", "chrome", "firefox"])
                .num_args(1..),
        )
        .arg(
            Arg::new("github")
                .long("github")
                .short('g')
                .help("Target Repository url")
                .required(false)
                .value_name("GITHUB_URL"),
        )
        .arg(
            Arg::new("outputpath")
                .long("outputpath")
                .short('p')
                .help("Saving direcotory, defulats to Documents")
                .required(false)
                .value_name("DIR_PATH"),
        )
        .arg(
            Arg::new("routine")
                .long("routine")
                .short('r')
                .value_name("SCHEDULE")
                .help("Routine schedule (day/week/month)")
                .required(false)
                .value_parser(["day", "week", "month"]),
        )
        .arg(
            Arg::new("count")
                .long("count")
                .short('c')
                .value_name("COUNT")
                .value_parser(clap::value_parser!(u32))
                .required(false)
                .help("count for routine"),
        )
        .try_get_matches()
        .unwrap_or_else(|e| e.exit());

    if verify_routine_count(&matches).is_err() {
        let _ = Command::new("")
            .error(
                clap::error::ErrorKind::InvalidSubcommand,
                "found no routine options. usage: --routine <day/week/month>.
                (not required,save one time only) , subcommand: --count <INTEGER> (default to one)",
            )
            .exit();
    }
    handle_matches(&matches)?;

    Ok(())
}

fn verify_routine_count(matches: &ArgMatches) -> Result<(), ()> {
    if let Some(_) = matches.get_one::<u32>("count")
        && matches.get_one::<String>("routine").is_none()
    {
        return Err(());
    }
    Ok(())
}

fn handle_matches(matches: &ArgMatches) -> io::Result<CliOptions> {
    let mut options = CliOptions::new();

    let os = match std::env::consts::OS {
        "linux" => SupportedOSs::Linux,
        "windows" => SupportedOSs::Windows,
        _ => {
            exit(1);
        }
    };
    options.supported_os = Some(os);

    // repo option match
    if let Some(gh_repository) = matches.get_one::<String>("github") {
        // to-do: add URL check
        options.github = Some(gh_repository.clone());
    }
    // browser options match
    if let Some(browsers) = matches.get_many::<String>("browser") {
        let mut selected_browsers: Vec<Browser> = Vec::new();
        for browser in browsers {
            match browser.as_str() {
                "brave" => selected_browsers.push(Browser::new(SupportedBrowsers::Brave)),
                "firefox" => selected_browsers.push(Browser::new(SupportedBrowsers::Firefox)),
                "chrome" => selected_browsers.push(Browser::new(SupportedBrowsers::Chrome)),
                _ => {
                    unreachable!("impossible to reach , the parsing is handled by clap parser")
                }
            }
        }
        options.browsers = selected_browsers;
    }
    // routine option match
    if let Some(routine) = matches.get_one::<String>("routine") {
        let count = matches.get_one::<u32>("count").copied().unwrap_or(1u32);

        options.routine = match routine.as_str() {
            "day" => Some(Routine::Day),
            "week" => Some(Routine::Week),
            "month" => Some(Routine::Month),
            _ => unreachable!("impossible to reach , the parsing is handled by clap parser"),
        };
        options.routine_count = count;
    }
    // outputpath option match
    if let Some(save_path) = matches.get_one::<String>("outputpath") {
        let mut save_path = save_path.clone();
        loop {
            if check_path(&save_path) {
                break;
            } else {
                // do we even have access to write in the directory?
                // i only checked the existence
                // to-do: check the write access

                let save_path_input: String =
                    get_input("The Save directory not found. please input a valid path");
                save_path = save_path_input;
            }
        }
        options.save_path = Some(save_path)
    }

    //println!("{:?}", options);
    search_browsers(&mut options);
    for browser in &options.browsers {
        chromium_parser(&browser);
    }
    Ok(options)
}
