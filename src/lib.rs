pub mod ui {
    use crate::service::filter_by_regex;
    use regex::Regex;
    use std::io;
    use tui::{
        backend::CrosstermBackend,
        layout::{Constraint, Direction, Layout},
        style::{Color, Modifier, Style},
        text::{Span, Spans},
        widgets::{Block, Borders, List, ListItem, ListState},
        Terminal,
    };

    pub fn start_ui(path: &String, regex: &String) -> Result<(), io::Error> {
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        terminal.clear()?;

        loop {
            terminal.draw(|rect| {
                let size = rect.size();
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Length(3)].as_ref())
                    .split(size);

                let entries = filter_by_regex(path, regex);
                let mut items: Vec<ListItem> = Vec::new();

                for entry in &entries {
                    let mut spans_vec: Vec<Span> = Vec::new();

                    if entry.matched_text.len() != 0 {
                        let dirs: Vec<&str> = entry.path.split("/").collect();
                        let current_sub_dir = dirs.get(dirs.len() - 1).unwrap();
                        let precedent_dirs_array = &dirs[0..dirs.len() - 1];

                        let mut path_without_current_dir = String::from("");
                        for dir in precedent_dirs_array {
                            path_without_current_dir.push_str(dir);
                            path_without_current_dir.push_str("/");
                        }

                        let span_raw = Span::raw(path_without_current_dir);
                        spans_vec.push(span_raw);

                        for current_match in &entry.matched_text {
                            let new_rgx = Regex::new(&regex).unwrap();
                            let splits: Vec<&str> = new_rgx.split(current_sub_dir).into_iter().collect();

                            let mut i = 0;
                            while i < splits.len() - 1 {
                                if i == 0 {
                                    let span_raw1 = Span::raw(splits.get(i).unwrap().to_owned());
                                    spans_vec.push(span_raw1);
                                }

                                let span_highlighted = Span::styled(
                                    current_match,
                                    Style::default()
                                        .fg(Color::LightYellow)
                                        .add_modifier(Modifier::BOLD),
                                );

                                let span_raw2 = Span::raw(splits.get(i + 1).unwrap().to_owned());

                                spans_vec.push(span_highlighted);
                                spans_vec.push(span_raw2);

                                i = i + 1;
                            }
                        }
                        items.push(ListItem::new(Spans::from(spans_vec)));
                    } else {
                        let span_raw = Span::raw(&entry.path);
                        items.push(ListItem::new(span_raw));
                    }
                }

                let mut state = ListState::default();
                state.select(Some(0));

                let list_block = Block::default()
                    .borders(Borders::ALL)
                    .title(format!("{}", regex));
                let list = List::new(items)
                    .style(Style::default().fg(Color::White))
                    .highlight_style(Style::default().bg(Color::Rgb(131, 113, 163)))
                    .highlight_symbol("> ");

                rect.render_stateful_widget(list.block(list_block), chunks[0], &mut state);
            });
        }

        terminal.clear()?;
        terminal.show_cursor()?;
        crossterm::terminal::disable_raw_mode()?;

        return Ok(());
    }
}

pub mod service {
    use regex::Regex;
    use std::{fs, process};

    #[derive(Debug)]
    pub struct Entry {
        pub path: String,
        pub matched_text: Vec<String>,
    }

    impl Entry {
        pub fn new(path: String) -> Entry {
            Entry {
                path,
                matched_text: Vec::new(),
            }
        }
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
            let mut new_entry = Entry::new(entry_display.to_owned());

            let dirs: Vec<&str> = entry_display.split("/").collect();
            let current_sub_dir = dirs.get(dirs.len() - 1).unwrap();

            let does_it_contain_filtered_text = rgx.is_match(current_sub_dir);
            if does_it_contain_filtered_text {
                let captures = rgx.captures(current_sub_dir).unwrap();

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
