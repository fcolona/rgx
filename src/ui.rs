use crate::service::{filter_by_regex, remove_dashes};
use regex::{Captures, Regex};
use std::{fs, io, io::stdout, io::Stdout, io::Write, thread, time::Duration, path::PathBuf};
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
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Terminal,
};
pub fn setup(path: &String, regex: &String) -> Result<(), io::Error> {
    let mut stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut stdout = io::stdout().into_raw_mode()?;
    let show_hidden_files = false;

    return start_ui(path, regex, show_hidden_files, terminal, stdout);
}

pub fn start_ui(
    path: &String,
    regex: &String,
    show_hidden_files: bool,
    mut terminal: Terminal<TermionBackend<RawTerminal<Stdout>>>,
    stdout: RawTerminal<Stdout>,
) -> Result<(), io::Error> {
    let entries = filter_by_regex(path, regex, show_hidden_files);
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

                    let span_raw2 =
                        Span::raw(remove_dashes(&splits.get(i + 1).unwrap().to_string()));
                    spans_vec.push(span_raw2);

                    i = i + 1;
                }
            }
            if !entry.content_matches.is_empty() {
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
            spans_vec.push(span_raw);

            if !entry.content_matches.is_empty() {
                let span_highlighted2 = Span::styled(
                    " *",
                    Style::default()
                        .fg(Color::Rgb(255, 93, 98))
                        .add_modifier(Modifier::BOLD),
                );
                spans_vec.push(span_highlighted2);
            }
            items.push(ListItem::new(Spans::from(spans_vec)))
        }
    }

    let mut stdin = termion::async_stdin().keys();
    let mut s = String::new();

    let mut state = ListState::default();
    state.select(Some(1));

    let selected_entry = entries.get(state.selected().unwrap() - 1).unwrap();
    let mut spans_vec: Vec<Span> = Vec::new();
    let splits: Vec<&str> = new_rgx.split(&selected_entry.content).into_iter().collect();

    let mut i = 0;
    while i < splits.len() - 1 {
        let span_raw1 = Span::raw(splits.get(i).unwrap().to_owned());
        spans_vec.push(span_raw1);

        let span_highlighted1 = Span::styled(
            selected_entry.content_matches.get(i).unwrap(),
            Style::default()
                .fg(Color::LightYellow)
                .add_modifier(Modifier::BOLD),
        );
        spans_vec.push(span_highlighted1);

        let span_raw2 = Span::raw(splits.get(i + 1).unwrap().to_owned());
        spans_vec.push(span_raw2);

        i = i + 1;
    }

    let text = vec![Spans::from(spans_vec)];

    let mut paragraph = Paragraph::new(text)
        .block(Block::default().title("Preview").borders(Borders::ALL))
        .style(Style::default().fg(Color::White).bg(Color::Black))
        .wrap(Wrap { trim: true });

    loop {
        let input = stdin.next();

        if let Some(Ok(key)) = input {
            match key {
                Key::Char('q') => break,
                Key::Char('j') => {
                    if state.selected().unwrap() < entries.len() {
                        state.select(Some(state.selected().unwrap() + 1));

                        let selected_entry = entries.get(state.selected().unwrap() - 1).unwrap();

                        if !selected_entry.is_a_directory {
                            let splits: Vec<&str> =
                                new_rgx.split(&selected_entry.content).into_iter().collect();
                            let mut spans_vec: Vec<Span> = Vec::new();

                            let mut i = 0;
                            while i < splits.len() - 1 {
                                let span_raw1 = Span::raw(splits.get(i).unwrap().to_owned());
                                spans_vec.push(span_raw1);

                                let span_highlighted1 = Span::styled(
                                    selected_entry.content_matches.get(i).unwrap(),
                                    Style::default()
                                        .fg(Color::LightYellow)
                                        .add_modifier(Modifier::BOLD),
                                );
                                spans_vec.push(span_highlighted1);

                                let span_raw2 = Span::raw(splits.get(i + 1).unwrap().to_owned());
                                spans_vec.push(span_raw2);

                                i = i + 1;
                            }
                            if splits.len() == 1 {
                                let span_raw1 = Span::raw(&selected_entry.content);
                                spans_vec.push(span_raw1);
                            }

                            let text = vec![Spans::from(spans_vec)];

                            paragraph = Paragraph::new(text)
                                .block(Block::default().title("Preview").borders(Borders::ALL))
                                .style(Style::default().fg(Color::White).bg(Color::Black))
                                .wrap(Wrap { trim: true });
                        } else {
                            paragraph = Paragraph::new("")
                                .block(Block::default().title("Preview").borders(Borders::ALL))
                                .style(Style::default().fg(Color::White).bg(Color::Black))
                                .wrap(Wrap { trim: true });
                        }
                    }
                }
                Key::Char('k') => {
                    if state.selected().unwrap() > 1 {
                        state.select(Some(state.selected().unwrap() - 1));

                        let selected_entry = entries.get(state.selected().unwrap() - 1).unwrap();

                        if !selected_entry.is_a_directory {
                            let splits: Vec<&str> =
                                new_rgx.split(&selected_entry.content).into_iter().collect();
                            let mut spans_vec: Vec<Span> = Vec::new();

                            let mut i = 0;
                            while i < splits.len() - 1 {
                                let span_raw1 = Span::raw(splits.get(i).unwrap().to_owned());
                                spans_vec.push(span_raw1);

                                let span_highlighted1 = Span::styled(
                                    selected_entry.content_matches.get(i).unwrap(),
                                    Style::default()
                                        .fg(Color::LightYellow)
                                        .add_modifier(Modifier::BOLD),
                                );
                                spans_vec.push(span_highlighted1);

                                let span_raw2 = Span::raw(splits.get(i + 1).unwrap().to_owned());
                                spans_vec.push(span_raw2);

                                i = i + 1;
                            }
                            if splits.len() == 1 {
                                let span_raw1 = Span::raw(&selected_entry.content);
                                spans_vec.push(span_raw1);
                            }

                            let text = vec![Spans::from(spans_vec)];

                            paragraph = Paragraph::new(text)
                                .block(Block::default().title("Preview").borders(Borders::ALL))
                                .style(Style::default().fg(Color::White).bg(Color::Black))
                                .wrap(Wrap { trim: true });
                        } else {
                            paragraph = Paragraph::new("")
                                .block(Block::default().title("Preview").borders(Borders::ALL))
                                .style(Style::default().fg(Color::White).bg(Color::Black))
                                .wrap(Wrap { trim: true });
                        }
                    }
                }
                Key::Char('g') => {
                    state.select(Some(0));
                }
                Key::Char('G') => {
                    let last_entry_index = entries.len();
                    state.select(Some(last_entry_index));
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

                        return start_ui(
                            &path_without_current_dir,
                            regex,
                            show_hidden_files,
                            terminal,
                            stdout,
                        );
                    } else {
                        let selected_entry = entries.get(state.selected().unwrap() - 1).unwrap();
                        let full_path = &entries.get(state.selected().unwrap() - 1).unwrap().path;
                        let is_empty = PathBuf::from(full_path).read_dir()?.next().is_none();

                        if selected_entry.is_a_directory && !is_empty{
                            drop(stdin);

                            return start_ui(
                                &selected_entry.path,
                                regex,
                                show_hidden_files,
                                terminal,
                                stdout,
                            );
                        }
                    }
                }
                Key::Char('h') => {
                    drop(stdin);

                    let full_path = &entries.get(state.selected().unwrap() - 1).unwrap().path;

                    let dirs: Vec<&str> = full_path.split("/").collect();

                    let current_sub_dir = dirs.get(dirs.len() - 1).unwrap();
                    let precedent_dirs_array = &dirs[0..dirs.len() - 2];

                    let mut path_without_current_dir = String::from("");
                    for dir in precedent_dirs_array {
                        path_without_current_dir.push_str(dir);
                        path_without_current_dir.push_str("/");
                    }

                    return start_ui(
                        &path_without_current_dir,
                        regex,
                        show_hidden_files,
                        terminal,
                        stdout,
                    );
                }
                Key::Char('n') => {
                    let selected_entry_index = state.selected().unwrap();

                    let mut i = selected_entry_index;
                    while i < entries.len() {
                        let current_entry = entries.get(i).unwrap();

                        if current_entry.matched_text.len() > 0 || current_entry.content_matches.len() > 0 {
                            state.select(Some(i + 1));

                            if !current_entry.is_a_directory {
                                let splits: Vec<&str> =
                                    new_rgx.split(&current_entry.content).into_iter().collect();
                                let mut spans_vec: Vec<Span> = Vec::new();

                                let mut i = 0;
                                while i < splits.len() - 1 {
                                    let span_raw1 = Span::raw(splits.get(i).unwrap().to_owned());
                                    spans_vec.push(span_raw1);

                                    let span_highlighted1 = Span::styled(
                                        current_entry.content_matches.get(i).unwrap(),
                                        Style::default()
                                            .fg(Color::LightYellow)
                                            .add_modifier(Modifier::BOLD),
                                    );
                                    spans_vec.push(span_highlighted1);

                                    let span_raw2 =
                                        Span::raw(splits.get(i + 1).unwrap().to_owned());
                                    spans_vec.push(span_raw2);

                                    i = i + 1;
                                }
                                if splits.len() == 1 {
                                    let span_raw1 = Span::raw(&current_entry.content);
                                    spans_vec.push(span_raw1);
                                }

                                let text = vec![Spans::from(spans_vec)];

                                paragraph = Paragraph::new(text)
                                    .block(Block::default().title("Preview").borders(Borders::ALL))
                                    .style(Style::default().fg(Color::White).bg(Color::Black))
                                    .wrap(Wrap { trim: true });
                                break;
                            } else {
                                paragraph = Paragraph::new("")
                                    .block(Block::default().title("Preview").borders(Borders::ALL))
                                    .style(Style::default().fg(Color::White).bg(Color::Black))
                                    .wrap(Wrap { trim: true });
                                break;
                            }
                        }
                        i = i + 1;
                    }
                }
                Key::Char('N') => {
                    let selected_entry_index = state.selected().unwrap() - 2;

                    let mut i = selected_entry_index;
                    while i > 0 {
                        let current_entry = entries.get(i).unwrap();

                        if current_entry.matched_text.len() > 0 || current_entry.content_matches.len() > 0 {                           state.select(Some(i + 1));

                            if !current_entry.is_a_directory {
                                let splits: Vec<&str> =
                                    new_rgx.split(&current_entry.content).into_iter().collect();
                                let mut spans_vec: Vec<Span> = Vec::new();

                                let mut i = 0;
                                while i < splits.len() - 1 {
                                    let span_raw1 = Span::raw(splits.get(i).unwrap().to_owned());
                                    spans_vec.push(span_raw1);

                                    let span_highlighted1 = Span::styled(
                                        current_entry.content_matches.get(i).unwrap(),
                                        Style::default()
                                            .fg(Color::LightYellow)
                                            .add_modifier(Modifier::BOLD),
                                    );
                                    spans_vec.push(span_highlighted1);

                                    let span_raw2 =
                                        Span::raw(splits.get(i + 1).unwrap().to_owned());
                                    spans_vec.push(span_raw2);

                                    i = i + 1;
                                }
                                if splits.len() == 1 {
                                    let span_raw1 = Span::raw(&current_entry.content);
                                    spans_vec.push(span_raw1);
                                }

                                let text = vec![Spans::from(spans_vec)];

                                paragraph = Paragraph::new(text)
                                    .block(Block::default().title("Preview").borders(Borders::ALL))
                                    .style(Style::default().fg(Color::White).bg(Color::Black))
                                    .wrap(Wrap { trim: true });
                                break;
                            } else {
                                paragraph = Paragraph::new("")
                                    .block(Block::default().title("Preview").borders(Borders::ALL))
                                    .style(Style::default().fg(Color::White).bg(Color::Black))
                                    .wrap(Wrap { trim: true });
                                break;
                            }
                        }
                        i = i - 1;
                    }
                }
                Key::Ctrl('h') => {
                    drop(stdin);

                    return start_ui(path, regex, !show_hidden_files, terminal, stdout);
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
                .direction(Direction::Horizontal)
                .constraints([Constraint::Ratio(4, 7), Constraint::Ratio(3, 7)].as_ref())
                .split(size);

            rect.render_stateful_widget(list.block(list_block), chunks[0], &mut state);
            rect.render_widget(paragraph.clone(), chunks[1]);
        });
    }

    terminal.clear()?;
    terminal.show_cursor()?;
    //terminal::disable_raw_mode()?;

    return Ok(());
}
