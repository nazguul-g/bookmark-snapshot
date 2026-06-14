// Handle parsing CLI arguments

use clap::{Arg, ArgMatches, Command, arg};

enum Browsers {
    Brave(Browser),
    Chrome(Browser),
    Tor(Browser),
    FireFox(Browser),
}
enum Routines {
    Week(u32),
    Month(u32),
    Day(u32),
}
struct Options {
    browsers: Vec<Browsers>,
    github: Option<String>,
    output_dir: Option<String>,
    routine: Option<Routines>,
}

// unless user specified the path, we use predefined paths for system to search.
struct Browser {
    paths: Vec<String>,
}
/*
main command : --browser <Browsers> (not required , looks for available browsers) , subcommand : --path <PATH> (not required , looks for default paths) ,
main command : --github <REPO_URL> (not required , save local only).
main command : --outputpath <DIR_PATH> (not required, save in Documents).
main command : --routine <DAY/WEEK/MONTH> , (not required, save one time only) , subcommand: --count <INTEGER> (not required, routine each DAY/WEEK/MONTH)
*/

// due to luck of documentation about clap derive, decided to use clap builder instead
pub fn cli() {
    let matches = Command::new("Bookmarks snapshot")
        .about("Automaticlly saves browser booksmarks")
        .version("0.1")
        .arg(
            Arg::new("browser")
                .long("browser")
                .short('b')
                .value_name("BROWSER")
                .help("Specify browser. Supported browser are : Brave, Tor, Firefox, Chrome")
                .required(false)
                .value_parser(["brave", "chrome", "tor", "firefox"])
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
                .value_name("SCHEDULE")
                .help("Routine schedule (day/week/month)")
                .required(false)
                .value_parser(["day", "week", "month"]),
        )
        .subcommand(
            Command::new("count")
                .about("count operation for routines")
                .arg(
                    Arg::new("count")
                        .long("count")
                        .short('c')
                        .value_name("COUNT")
                        .value_parser(clap::value_parser!(u32))
                        .required(false)
                        .help("count for routine"),
                ),
        )
        .subcommand(
            Command::new("path").about("path for specific browser").arg(
                Arg::new("path")
                    .long("browserpath")
                    .value_name("BROWSER_PATH")
                    .required(false)
                    .help("browserpath"),
            ),
        )
        .try_get_matches()
        .unwrap_or_else(|e| e.exit());
    if let Err(e) = validate_routine_count(&matches) {
        let _ = Command::new("bookmark snapshot")
            .error(clap::error::ErrorKind::InvalidSubcommand, e)
            .exit();
    }
}
fn validate_routine_count(matches: &ArgMatches) -> Result<(), String> {
    if matches.subcommand_name() == Some("count") && matches.get_one::<String>("routine").is_none()
    {
        return Err("The 'count' subcommand can only be used with --routine option".to_string());
    }
    Ok(())
}
fn handle_matches(matches: &ArgMatches) {
    let mut options = Options {
        browsers: Vec::new(),
        github: None,
        output_dir: None,
        routine: None,
    };
    if let Some(browsers) = matches.get_many::<String>("browser") {
        for browser in browsers {}
    }
}

// get browser default paths
fn get_browser_path() {}

