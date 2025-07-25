use crate::ui::app::{App, CurrentScreen, CurrentlyEditing};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, palette::tailwind},
    text::{Line, Span, Text},
    widgets::{
        Block, BorderType, Borders, Cell, HighlightSpacing, List, ListItem, Paragraph, Row, Table,
        TableState,
    },
};
use rusqlite;
use todo::db::list_all_notes;
use todo::types::Note;

pub fn ui(f: &mut Frame, app: &App) {
    let current_screen = &app.current_screen;
    // eprintln!("Debug: current_screen = {:?}", &current_screen);
    match current_screen {
        CurrentScreen::ListNotes => {
            list_notes(f, app);
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

fn list_notes(f: &mut Frame, app: &App) {
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
                render_table(f, app, chunk[1], note_list);
            }
        }
        Err(_) => {
            render_error(f, chunk[1], String::from("Error fetching list!"));
        }
    }

    list_notes_footer(f, chunk[2]);
}

fn list_renderer(note_list: Vec<Note>, f: &mut Frame, area: Rect) {
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
    let render_list =
        List::new(note_span_list).block(Block::default().borders(Borders::ALL).title("Notes"));

    f.render_widget(render_list, area);
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

fn render_table(f: &mut Frame, app: &App, area: Rect, list: Vec<Note>) {
    let rows: Vec<Row> = list
        .iter()
        .enumerate()
        .map(|(i, note)| {
            let color = match i % 2 {
                0 => tailwind::SLATE.c950,
                _ => tailwind::SLATE.c900,
            };
            Row::new(vec![
                Cell::from(Text::from(format!("\n{}\n", note.id.to_string()))),
                Cell::from(Text::from(format!("\n{}\n", note.title.clone()))),
            ])
            .style(Style::new().fg(tailwind::SLATE.c200).bg(color))
            .height(3)
        })
        .collect();

    let bar = " █ ";
    let table = Table::new(
        rows,
        [Constraint::Percentage(10), Constraint::Percentage(90)],
    )
    .header(
        Row::new(vec!["ID", "Title"]).style(
            Style::default()
                .bg(tailwind::BLUE.c900)
                .fg(tailwind::SLATE.c200),
        ),
    )
    .block(Block::default().borders(Borders::ALL).title("Notes"))
    .row_highlight_style(
        Style::default()
            .fg(tailwind::BLUE.c400)
            .add_modifier(Modifier::REVERSED),
    )
    .highlight_symbol(Text::from(vec!["".into(), bar.into(), "".into()]))
    .highlight_spacing(HighlightSpacing::Always);

    f.render_stateful_widget(
        table,
        area,
        &mut TableState::default().with_selected(app.currently_selected),
    );
}

fn list_notes_footer(f: &mut Frame, area: Rect) {
    let info_text = "(Esc) quit | (↑) move up | (↓) move down | (e) edit | (d) delete";
    let info_footer = Paragraph::new(info_text)
        .style(
            Style::new()
                .fg(tailwind::SLATE.c200)
                .bg(tailwind::SLATE.c950),
        )
        .centered()
        .block(
            Block::bordered()
                .border_type(BorderType::Double)
                .border_style(Style::new().fg(tailwind::BLUE.c400)),
        );

    f.render_widget(info_footer, area);
}
