use std::{env, process};
use rgx::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("ERROR: not enough arguments");
        process::exit(1)
    }

    let regex = &args[1];
    let path = &args[2];

    let filtered_texts = filter_by_regex(path, regex);
    for text in filtered_texts {
        println!("{}", text);
    }

    //println!("regex: {}", regex);
    //println!("path: {}", path);
    //println!("dir: {:?}", dir);
}
