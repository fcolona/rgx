use std::{fs, process};
use regex::Regex;

pub fn filter_by_regex(path: &String, regex: &String) -> Vec<String> {
    let regex = format!(r"{}", regex);
    let rgx = Regex::new(&regex).unwrap_or_else( |_err| {
        println!("ERROR: not a valid regex");
        process::exit(1)
    });
    let mut matched_texts = Vec::new();

    for entry in fs::read_dir(path).unwrap_or_else(|_err| {
        println!("ERROR: not a valid path");
        process::exit(1)
    }) {
        let entry_display = &entry.unwrap().path().display().to_string();

        let does_it_contain_filtered_text = rgx.is_match(entry_display);
        if does_it_contain_filtered_text {
            matched_texts.push(entry_display.to_owned());
        }
        //println!("{:?} --- {:?}", entry_display, does_it_contain_filtered_text);
    }

    return matched_texts;
}
