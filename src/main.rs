use std::{io, env, process};
use rgx::ui::setup;

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
