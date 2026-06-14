// Handle parsing CLI arguments

use clap::{Arg, ArgMatches, Command, arg, builder::Str};
// Unless user specified the path, we use predefined paths for system to search.
// the path logic is broken
// decision:  
//  automaticlly search for browsers 
//  if none found we ask user to input path 
#[derive(Debug)]

enum Browsers {
    Brave(String),
    Chrome(String),
    Tor(String),
    FireFox(String),
}
#[derive(Debug)]
struct Options {
    browsers: Vec<Browsers>,
    github: Option<String>,
    output_dir: Option<String>,
    routine: Option<Routines>,
}
#[derive(Debug)]
enum Routines {
    Week(u32),
    Month(u32),
    Day(u32),
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
        .arg(
            Arg::new("path")
                .long("browserpath")
                .value_name("BROWSER_PATH")
                .required(false)
                .help("browser path"),
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
    if let Err(e) = validate_routine_count(&matches) {
        let _ = Command::new("bookmark snapshot")
            .error(clap::error::ErrorKind::InvalidSubcommand, e)
            .exit();
    }
    println!("{:?}", handle_matches(&matches))
}
fn validate_routine_count(matches: &ArgMatches) -> Result<(), String> {
    if matches.subcommand_name() == Some("count") && matches.get_one::<String>("routine").is_none()
    {
        return Err("The 'count' subcommand can only be used with --routine option".to_string());
    }
    Ok(())
}

fn handle_matches(matches: &ArgMatches) -> Options {
    let mut options = Options {
        browsers: Vec::new(),
        github: None,
        output_dir: None,
        routine: None,
    };
    // github repo match
    if let Some(github) = matches.get_one::<String>("github") {
        options.github = Some(github.clone());
    }
    // output path match
    if let Some(outputpath) = matches.get_one::<String>("outputpath") {
        options.output_dir = Some(outputpath.clone());
    }

    // routine matches
    if let Some(routine) = matches.get_one::<String>("routine") {
        // if let Some(count) = matches.get_one::<u32>("count") {
        //     match routine.as_str() {
        //         "day" => options.routine = Some(Routines::Day(count.clone())),
        //         "week" => options.routine = Some(Routines::Week(count.clone())),
        //         "month" => options.routine = Some(Routines::Month(count.clone())),
        //         _ => (),
        //     }
        // } else {
        //     match routine.as_str() {
        //         "day" => options.routine = Some(Routines::Day(1)),
        //         "week" => options.routine = Some(Routines::Week(1)),
        //         "month" => options.routine = Some(Routines::Month(1)),
        //         _ => (),
        //     }
        // }
        //
        let count = matches.get_one("count").copied().unwrap_or(1);

        options.routine = match routine.as_str() {
            "day" => Some(Routines::Day(count)),
            "week" => Some(Routines::Week(count)),
            "month" => Some(Routines::Month(count)),
            _ => None,
        };
    }

    // browsers match
    if let Some(browsers) = matches.get_many::<String>("browser") {
        
        let path = "".to_string();

        for browser in browsers {
            match browser.as_str() {
                "brave" => options.browsers.push(Browsers::Brave(path.clone())),
                "tor" => options.browsers.push(Browsers::Tor(path.clone())),
                "firefox" => options.browsers.push(Browsers::FireFox(path.clone())),
                "chrome" => options.browsers.push(Browsers::Chrome(path.clone())),
                _ => (),
            }
        }
    }

    options
}

