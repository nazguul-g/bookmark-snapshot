// Handle parsing CLI arguments 


use clap::{Command, arg};

enum Browsers {
    Brave,
    Chrome,
    Tor,
    FireFox,
}

/*
main command : --browser <Browsers> (not required , looks for available browsers) , subcommand : --path <PATH> (not required , looks for default paths) ,
main command : --github <REPO_URL> (not required , save local only).
main command : --outputpath <DIR_PATH> (not required, save in Documents).
main command : --routine <DAY/WEEK/MONTH> , (not required, save one time only) , subcommand: --count <INTEGER> (not required, routine each DAY/WEEK/MONTH)


*/
pub fn cli() {
    let matches = Command::new("Bookmarks snapshot")
        .about("Automaticlly saves browser booksmarks")
        .version("0.1")
        .arg(arg!(--github <REPOSITORY_URL>).required(false))
        .arg(arg!(--browsers));
}
