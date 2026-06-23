// Parses bookmarks for two different engines : gecko and chromium
// Initial anylysis found that gecko uses sqlite representation and chromuim uses json format

// in gecko based browsers, for safely parsing, copy the file into temp location , and opearte on it. cuz if the browser is running it might lock the DB
// sqlite table where bookmarks resides is moz_bookmarks for mozzila browser
// use stringlossy , some names are uncompatible with String
// 
pub mod chromium;
pub mod gecko;
mod types;