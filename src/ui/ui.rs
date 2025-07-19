use crate::ui::app::{App, CurrentScreen, CurrentlyEditing};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
};
use rusqlite;
use todo::db::list_all_notes;
use todo::types::Note;

pub fn ui(f: &mut Frame, app: &App) {
    let current_screen = &app.current_screen;
    // eprintln!("Debug: current_screen = {:?}", &current_screen);
    match current_screen {
        CurrentScreen::ListNotes => {
            eprintln!("ListNotes");
            list_notes(f);
        }
        CurrentScreen::CreateNote => {
            todo!();
        }
        CurrentScreen::Editing => {
            todo!();
        }
        CurrentScreen::Exiting => {
            todo!();
        }
    }
}

fn render_title(f: &mut Frame, title: String, title_chunk: Rect) {
    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());
    let title =
        Paragraph::new(Text::styled(title, Style::default().fg(Color::Green))).block(title_block);
    // eprintln!("Rendering title");
    f.render_widget(title, title_chunk);
}

fn list_notes(f: &mut Frame) {
    // eprintln!("Creating chunk");
    let chunk = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(f.area());

    // NOTE: Rendering Title Block
    // eprintln!("Creating title");
    let title = String::from("List All Notes");
    let title_chunk = chunk[0];
    render_title(f, title, title_chunk);

    let note_list: Result<Vec<Note>, rusqlite::Error> = list_all_notes();

    match note_list {
        Ok(note_list) => {
            if note_list.is_empty() {
                render_error(f, chunk[1], String::from("No notes to show!"));
            } else {
                let note_span_list: Vec<ListItem> = note_list
                    .iter()
                    .enumerate()
                    .map(|(index, note)| {
                        ListItem::new(Line::from(vec![
                            Span::raw(format!("{}", index + 1)),
                            Span::styled(note.title.clone(), Style::default().fg(Color::Blue)),
                        ]))
                    })
                    .collect();
                let render_list = List::new(note_span_list)
                    .block(Block::default().borders(Borders::ALL).title("Notes"));

                f.render_widget(render_list, chunk[1]);
            }
        }
        Err(_) => {
            render_error(f, chunk[1], String::from("Error fetching list!"));
        }
    }
}

fn render_error(f: &mut Frame, area: Rect, error_msg: String) {
    let vertical_padding = area.height.saturating_sub(1) / 2;
    let mut lines = Vec::new();

    for _ in 0..vertical_padding {
        lines.push(Line::default());
    }

    lines.push(Line::from(Span::styled(
        error_msg,
        Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
    )));

    f.render_widget(
        Paragraph::new(lines)
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).title("Info")),
        area,
    );
}
