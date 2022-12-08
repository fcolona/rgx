#![feature(io_error_more)]
#![allow(warnings, unused)]

use std::{io, env, process};
use ui::setup;

pub mod ui;
mod service;

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("ERROR: not enough arguments");
        process::exit(1)
    }

    let regex = &args[1];
    let path = &args[2];

    let result = setup(path, regex);    
    return result;
}
