//Libraries
use console::{Term, style};
use dialoguer::Select;
use dirs;
use std::fmt;
use std::str::FromStr;
use todo::{db, types};

// Modules
mod ui;

enum Choices {
    AddNotes,
    List,
    Exit,
}

impl FromStr for Choices {
    type Err = ();
    fn from_str(choice: &str) -> Result<Self, Self::Err> {
        match choice {
            "Add new notes" => Ok(Choices::AddNotes),
            "List all notes" => Ok(Choices::List),
            "exit" => Ok(Choices::Exit),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Choices {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            Choices::AddNotes => "Add new notes",
            Choices::List => "List all notes",
            Choices::Exit => "exit",
        };
        write!(f, "{}", label)
    }
}

fn main() {
    println!(
        "{}",
        style("Welcome to notes taker cli!\n").bold().blink().red()
    );

    let choice_list = vec![Choices::AddNotes, Choices::List, Choices::Exit];
    // db::dummy_notes_to_test();
    let choice = Select::new()
        .with_prompt("Choose...")
        .items(&choice_list)
        .default(0)
        .interact()
        .unwrap();

    let selected_choice = &choice_list[choice];
    match selected_choice {
        Choices::AddNotes => {
            ui::index::initializeTerminal(ui::app::CurrentScreen::CreateNote);
        }
        Choices::List => {
            ui::index::initializeTerminal(ui::app::CurrentScreen::ListNotes);
        }
        Choices::Exit => std::process::exit(0),
    }
}
