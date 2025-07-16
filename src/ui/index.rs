use crate::ui::app::{App, CurrentScreen, CurrentlyEditing};
use crate::ui::ui::ui;
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::Terminal;
use ratatui::backend::{Backend, CrosstermBackend};
use ratatui::crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::{EnterAlternateScreen, enable_raw_mode};
use ratatui::crossterm::terminal::{LeaveAlternateScreen, disable_raw_mode};
use std::io;

pub fn initializeTerminal(screen: CurrentScreen) -> Result<()> {
    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let mut app = App::new();
    // eprintln!("screen from main = {:?}", screen);
    app.current_screen = screen;
    // eprintln!(
    //     "current screen after initializing = {:?}",
    //     app.current_screen
    // );
    let res = run_app(&mut terminal, &mut app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Ok(do_print) = res {
        if do_print {
            todo!();
        }
    } else if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    loop {
        terminal.draw(|f| ui(f, app))?;
        // --snip--
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                // Skip events that are not KeyEventKind::Press
                continue;
            }
            // match app.current_screen {
            //     CurrentScreen::Main => match key.code {
            //         KeyCode::Char('e') => {
            //             app.current_screen = CurrentScreen::Editing;
            //             app.currently_editing = Some(CurrentlyEditing::Key);
            //         }
            //         KeyCode::Char('q') => {
            //             app.current_screen = CurrentScreen::Exiting;
            //         }
            //         _ => {}
            //     },
            //     CurrentScreen::Exiting => match key.code {
            //         KeyCode::Char('y') => {
            //             return Ok(true);
            //         }
            //         KeyCode::Char('n') | KeyCode::Char('q') => {
            //             return Ok(false);
            //         }
            //         _ => {}
            //     },
            //     CurrentScreen::Editing if key.kind == KeyEventKind::Press => match key.code {
            //         KeyCode::Enter => {
            //             if let Some(editing) = &app.currently_editing {
            //                 match editing {
            //                     CurrentlyEditing::Key => {
            //                         app.currently_editing = Some(CurrentlyEditing::Value);
            //                     }
            //                     CurrentlyEditing::Value => {
            //                         app.save_key_value();
            //                         app.current_screen = CurrentScreen::Main;
            //                     }
            //                 }
            //             }
            //         }
            //         KeyCode::Backspace => {
            //             if let Some(editing) = &app.currently_editing {
            //                 match editing {
            //                     CurrentlyEditing::Key => {
            //                         app.key_input.pop();
            //                     }
            //                     CurrentlyEditing::Value => {
            //                         app.value_input.pop();
            //                     }
            //                 }
            //             }
            //         }
            //         KeyCode::Esc => {
            //             app.current_screen = CurrentScreen::Main;
            //             app.currently_editing = None;
            //         }
            //         KeyCode::Tab => {
            //             app.toggle_editing();
            //         }
            //         KeyCode::Char(value) => {
            //             if let Some(editing) = &app.currently_editing {
            //                 match editing {
            //                     CurrentlyEditing::Key => {
            //                         app.key_input.push(value);
            //                     }
            //                     CurrentlyEditing::Value => {
            //                         app.value_input.push(value);
            //                     }
            //                 }
            //             }
            //         }
            //         _ => {}
            //     },
            //     _ => {}
            // }
        }
        // --snip--
    }
}
