// Handle parsing CLI arguments

use std::{env, process::exit};

use clap::{Arg, ArgMatches, Command};

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
    
}
