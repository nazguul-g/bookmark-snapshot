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

use crate::types::Options;


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
// Search browsers
// Option 1
// use preset of paths, and search
// Option 2
// use find command to find specifics
// Criticizing
// Option two seems not ideal. Because how do we identify each browser alone,
// we could say just find by "name of binary", but binary path and path of target data aren't the same.
//
// Option one seems like more static, does not dynamic searches for paths
//
// Final decision : we use both
//
// use a set of Known Config folders for each browser , like for brave they usually name the config folder by "BraveSoftware"
// , we also set a list of Config location to make the search faster.
//
// Each browser have its unique structure
// the actual bookmark path differs from browser to browser, gecko or chromium have nothing to do with that.
//

pub fn find_browsers () {}