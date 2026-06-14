// Handle writing and reading from files
// read and write config files
// making directories , and browser related stuff
// find browsers , and related paths
// find saved bookmarks paths
// supported engines
//      chromuim
//      gecko
// supported browsers are
//      brave
//      chrome
//      tor
//      firefox

use crate::cli::{Options, Routines};

// config file strategy
// new config :
//  create new dir holds the name of software
//  create the config needed
// updating the config :
//  search for the config in the assigned path
//  add new backups subdir
//  name the add unix timestamp as file format .bak.unixtimestamp
//  back to the main dir and overwrite the file
// fallbacks :
//  existing directory :
//      ask user directly for overwrite
struct Config {
    options: Options,
}
