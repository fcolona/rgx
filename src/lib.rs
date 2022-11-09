pub mod ui {
    use crate::service::filter_by_regex;
    use std::io;
    use tui::{
        backend::CrosstermBackend,
        layout::{Alignment, Constraint, Direction, Layout},
        style::{Color, Modifier, Style},
        text::{Span, Spans},
        widgets::{
            Block, BorderType, Borders, Cell, List, ListItem, ListState, Paragraph, Row, Table,
            Tabs,
        },
        Terminal,
    };

    pub fn start_ui(path: &String, regex: &String) -> Result<(), io::Error> {
        let mut stdout = io::stdout();
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
                let mut items = Vec::new();
                for entry in entries {
                    println!("{:#?}", entry);
                    items.push(ListItem::new(entry.path));
                }

                let list = List::new(items)
                    .block(Block::default().title("Regex").borders(Borders::ALL))
                    .style(Style::default().fg(Color::White))
                    .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
                    .highlight_symbol(">>");
                rect.render_widget(list, chunks[0]);
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

                for capture in captures.iter() {
                    new_entry.matched_text.push(capture.unwrap().as_str().to_owned());
                }
                entries.push(new_entry);
            }else {
                entries.push(new_entry)
            }
            //println!("{:?} --- {:?}", entry_display, does_it_contain_filtered_text);
        }

        return entries;
    }
}
