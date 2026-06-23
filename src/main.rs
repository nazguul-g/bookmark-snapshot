#![allow(unused)]

use crate::cli::cli;

mod cli;
mod io;
mod types;
mod parser;

fn main() {
    cli();
}
