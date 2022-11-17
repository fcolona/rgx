#![feature(io_error_more)]
#![allow(warnings, unused)]

//TODO: Learn about callbacks in rust
//TODO: try to understand why first key press after reloading ui 'bugs'
pub mod ui {
    use crate::service::{filter_by_regex, remove_dashes};
    use regex::Regex;
    use std::{fs, io, io::stdout, io::Stdout, io::Write, thread, time::Duration};
    use termion::{
        event::Key,
        input::TermRead,
        raw::{IntoRawMode, RawTerminal},
    };
    use tui::{
        backend::TermionBackend,
        layout::{Constraint, Direction, Layout},
        style::{Color, Modifier, Style},
        text::{Span, Spans},
        widgets::{Block, Borders, List, ListItem, ListState},
        Terminal,
    };
    pub fn setup(path: &String, regex: &String) -> Result<(), io::Error> {
        let mut stdout = io::stdout().into_raw_mode()?;
        let backend = TermionBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        let mut stdout = io::stdout().into_raw_mode()?;

        return start_ui(path, regex, terminal, stdout);
    }

    pub fn start_ui(
        path: &String,
        regex: &String,
        mut terminal: Terminal<TermionBackend<RawTerminal<Stdout>>>,
        stdout: RawTerminal<Stdout>,
    ) -> Result<(), io::Error> {
        let entries = filter_by_regex(path, regex);
        let new_rgx = Regex::new(&regex).unwrap();

        //This hole block is to highlight the matches
        let mut items: Vec<ListItem> = Vec::new();

        let go_back_span = Span::raw("(..)");
        items.push(ListItem::new(go_back_span));

        for entry in &entries {
            let mut spans_vec: Vec<Span> = Vec::new();
            let dirs: Vec<&str> = entry.path.split("/").collect();
            let current_sub_dir = dirs.get(dirs.len() - 1).unwrap();
            let precedent_dir = dirs.get(dirs.len() - 2).unwrap();

            let span_raw = Span::raw(format!("/{}/", precedent_dir));

            if entry.matched_text.len() != 0 {
                spans_vec.push(span_raw);

                for current_match in &entry.matched_text {
                    let splits: Vec<&str> = new_rgx.split(current_sub_dir).into_iter().collect();

                    let mut i = 0;
                    while i < splits.len() - 1 {
                        if i == 0 {
                            let span_raw1 = Span::raw(splits.get(i).unwrap().to_owned());
                            spans_vec.push(span_raw1);
                        }

                       if !splits.get(i).unwrap().eq(&"-") {
                            let span_highlighted1 = Span::styled(
                                current_match,
                                Style::default()
                                    .fg(Color::LightYellow)
                                    .add_modifier(Modifier::BOLD),
                            );
                            spans_vec.push(span_highlighted1);
                        } 

                        let span_raw2 = Span::raw(remove_dashes(&splits.get(i + 1).unwrap().to_string()));
                        spans_vec.push(span_raw2);

                        i = i + 1;
                    }
                }

                if entry.does_content_have_matches {
                    let span_highlighted2 = Span::styled(
                        " *",
                        Style::default()
                            .fg(Color::Rgb(255, 93, 98))
                            .add_modifier(Modifier::BOLD),
                    );
                    spans_vec.push(span_highlighted2);
                }
                items.push(ListItem::new(Spans::from(spans_vec)));
            } else {
                let span_raw = Span::raw(format!("/{}/{}", precedent_dir, current_sub_dir));
                items.push(ListItem::new(span_raw))
            }
        }

        let mut stdin = termion::async_stdin().keys();
        let mut s = String::new();

        let mut state = ListState::default();
        state.select(Some(1));
        loop {
            let input = stdin.next();

            if let Some(Ok(key)) = input {
                match key {
                    Key::Char('q') => break,
                    Key::Char('j') => {
                        if state.selected().unwrap() < entries.len() {
                            state.select(Some(state.selected().unwrap() + 1));
                        }
                    }
                    Key::Char('k') => {
                        if state.selected().unwrap() > 0 {
                            state.select(Some(state.selected().unwrap() - 1));
                        }
                    }
                    Key::Char('g') => {
                        state.select(Some(0));
                    }
                    Key::Char('l') => {
                        if (state.selected().unwrap()) == 0 {
                            drop(stdin);

                            let full_path = &entries.get(state.selected().unwrap()).unwrap().path;

                            let dirs: Vec<&str> = full_path.split("/").collect();

                            let current_sub_dir = dirs.get(dirs.len() - 1).unwrap();
                            let precedent_dirs_array = &dirs[0..dirs.len() - 2];

                            let mut path_without_current_dir = String::from("");
                            for dir in precedent_dirs_array {
                                path_without_current_dir.push_str(dir);
                                path_without_current_dir.push_str("/");
                            }

                            return start_ui(&path_without_current_dir, regex, terminal, stdout);
                        } else {
                            let selected_entry =
                                entries.get(state.selected().unwrap() - 1).unwrap();

                            if selected_entry.is_a_directory {
                                drop(stdin);

                                return start_ui(&selected_entry.path, regex, terminal, stdout);
                            }
                        }
                    }
                    _ => {}
                }
            }

            terminal.draw(|rect| {
                let items = items.clone();
                let list = List::new(items)
                    .style(Style::default().fg(Color::White))
                    .highlight_style(Style::default().bg(Color::Rgb(131, 113, 163)))
                    .highlight_symbol("> ");

                let list_block = Block::default()
                    .borders(Borders::ALL)
                    .title(format!("{}", regex));

                let size = rect.size();
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Length(3)].as_ref())
                    .split(size);

                rect.render_stateful_widget(list.block(list_block), chunks[0], &mut state);
            });
        }

        terminal.clear()?;
        terminal.show_cursor()?;
        //terminal::disable_raw_mode()?;

        return Ok(());
    }
}

pub mod service {
    use regex::Regex;
    use std::{fs, io::ErrorKind, process};

    #[derive(Debug)]
    pub struct Entry {
        pub path: String,
        pub matched_text: Vec<String>,
        pub is_a_directory: bool,
        pub does_content_have_matches: bool,
    }

    impl Entry {
        pub fn new(path: String, is_a_directory: bool, does_content_have_matches: bool) -> Entry {
            Entry {
                path,
                matched_text: Vec::new(),
                is_a_directory,
                does_content_have_matches,
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

    pub fn filter_by_regex(path: &String, regex: &String) -> Vec<Entry> {
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
            let mut new_entry = Entry::new(entry_display.to_owned(), false, false);

            let dirs: Vec<&str> = entry_display.split("/").collect();
            let current_sub_dir = dirs.get(dirs.len() - 1).unwrap();

            let content = fs::read_to_string(entry_display).unwrap_or_else(|err| {
                if err.kind().eq(&ErrorKind::IsADirectory) {
                    new_entry.is_a_directory = true
                }
                return String::from("");
            });

            if rgx.is_match(&content) {
                new_entry.does_content_have_matches = true
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
        }

        return entries;
    }
}
