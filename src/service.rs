use regex::{Captures, Regex};
use std::{fmt::format, fs, io::ErrorKind, process};

#[derive(Debug)]
pub struct Entry {
    pub path: String,
    pub matched_text: Vec<String>,
    pub is_a_directory: bool,
    pub content: String,
    pub content_matches: Vec<String>,
}

impl Entry {
    pub fn new(path: String, is_a_directory: bool) -> Entry {
        Entry {
            path,
            matched_text: Vec::new(),
            is_a_directory,
            content: String::from(""),
            content_matches: Vec::new(),
        }
    }
}

pub fn remove_dashes(string: &String) -> String {
    let mut string_builder = String::from("");

    for char in string.chars() {
        if !char.eq(&'-') {
            string_builder.push(char);
        }
    }
    return string_builder;
}

pub fn filter_by_regex(path: &String, regex: &String, show_hidden_files: bool) -> Vec<Entry> {
    let regex = format!(r"{}", regex);
    let rgx = Regex::new(&regex).unwrap_or_else(|_err| {
        println!("ERROR: not a valid regex");
        process::exit(1)
    });
    let mut entries = Vec::new();

    for entry in fs::read_dir(path).unwrap_or_else(|_err| {
        println!("ERROR: not a valid path");
        process::exit(1)
    }) {
        let entry_display = &entry.unwrap().path().display().to_string();
        let mut new_entry = Entry::new(entry_display.to_owned(), false);
        let dirs: Vec<&str> = entry_display.split("/").collect();
        let current_sub_dir = dirs.get(dirs.len() - 1).unwrap();

        if show_hidden_files == true {
            let content = fs::read_to_string(entry_display).unwrap_or_else(|err| {
                if err.kind().eq(&ErrorKind::IsADirectory) {
                    new_entry.is_a_directory = true
                }
                return String::from("");
            });

            if content != "" {
                let content = content.replace('\n', "");

                new_entry.content = content;
                let captures_iter = rgx.captures_iter(&new_entry.content);
                for capture in captures_iter {
                    new_entry
                        .content_matches
                        .push(capture.get(0).unwrap().as_str().to_owned());
                }
            }

            let does_it_contain_filtered_text = rgx.is_match(current_sub_dir);
            if does_it_contain_filtered_text {
                let current_sub_dir = remove_dashes(&current_sub_dir.to_string());
                let captures = rgx.captures(&current_sub_dir).unwrap();

                let mut i = 0;
                while i < captures.len() {
                    if i == 0 {
                        new_entry
                            .matched_text
                            .push(captures.get(i).unwrap().as_str().to_owned());
                    }

                    for saved_match in new_entry.matched_text.clone() {
                        if !saved_match.contains(&captures.get(i).unwrap().as_str().to_owned()) {
                            new_entry
                                .matched_text
                                .push(captures.get(i).unwrap().as_str().to_owned());
                        }
                    }

                    i = i + 1;
                }
                entries.push(new_entry);
            } else {
                entries.push(new_entry)
            }
        } else {
            if !current_sub_dir.chars().nth(0).unwrap().eq(&'.') {
                let content = fs::read_to_string(entry_display).unwrap_or_else(|err| {
                    if err.kind().eq(&ErrorKind::IsADirectory) {
                        new_entry.is_a_directory = true
                    }
                    return String::from("");
                });

                let content = format!(r"{}", content);

                if content != "" {
                    let content = content.replace('\n', "");
                    new_entry.content = content;
                    let captures_iter = rgx.captures_iter(&new_entry.content);
                    for capture in captures_iter {
                        new_entry
                            .content_matches
                            .push(capture.get(0).unwrap().as_str().to_owned());
                    }
                }
                if new_entry.path.eq("/home/felipe/crontab-wallpaper-shift") {
                    //println!("{:?}", new_entry.content_matches);
                }

                let does_it_contain_filtered_text = rgx.is_match(current_sub_dir);
                if does_it_contain_filtered_text {
                    let current_sub_dir = remove_dashes(&current_sub_dir.to_string());
                    let captures = rgx.captures(&current_sub_dir).unwrap();

                    let mut i = 0;
                    while i < captures.len() {
                        if i == 0 {
                            new_entry
                                .matched_text
                                .push(captures.get(i).unwrap().as_str().to_owned());
                        }

                        for saved_match in new_entry.matched_text.clone() {
                            if !saved_match.contains(&captures.get(i).unwrap().as_str().to_owned())
                            {
                                new_entry
                                    .matched_text
                                    .push(captures.get(i).unwrap().as_str().to_owned());
                            }
                        }

                        i = i + 1;
                    }
                    entries.push(new_entry);
                } else {
                    entries.push(new_entry)
                }
            }
        }
    }

    return entries;
}
