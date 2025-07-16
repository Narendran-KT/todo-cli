use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
};

use crate::ui::app::{App, CurrentScreen, CurrentlyEditing};

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
}
