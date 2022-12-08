#![feature(io_error_more)]
#![allow(warnings, unused)]

use std::{env, io, process};
use ui::setup;

mod service;
pub mod ui;

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 1 {
        println!("ERROR: not enough arguments");
        process::exit(1)
    }

    let regex = &args[1];

    let path = if args.len() == 2 || args[2].eq(".") {
        let path_buff = env::current_dir();
        let path = path_buff.unwrap().display().to_string();
        path
    } else {
        args[2].clone()
    };

    let result = setup(&path, regex);
    return result;
}
