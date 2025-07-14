//Libraries
use console::{Term, style};
use dialoguer::Select;
use dirs;
use rusqlite::{Connection, Result};
use std::fmt;
use std::str::FromStr;

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
    let mut connection = connect_db();
    println!(
        "{}",
        style("Welcome to notes taker cli!\n").bold().blink().red()
    );

    let choice_list = vec![Choices::AddNotes, Choices::List, Choices::Exit];
    let choice = Select::new()
        .with_prompt("Choose...")
        .items(&choice_list)
        .default(0)
        .interact()
        .unwrap();

    let selected_choice = &choice_list[choice];
    match selected_choice {
        Choices::AddNotes => {
            ui::index::initializeTerminal();
        }
        Choices::List => {
            todo!()
        }
        Choices::Exit => std::process::exit(0),
    }
}

fn get_db_path() -> std::path::PathBuf {
    let mut path: std::path::PathBuf =
        dirs::data_dir().unwrap_or_else(|| std::path::PathBuf::from("."));
    path.push("notes-cli");
    println!("Your notes will be saving to {}", path.display());
    std::fs::create_dir_all(&path).unwrap();
    path.push("notes.db");
    path
}

fn connect_db() -> Result<Connection> {
    let db_path = get_db_path();
    let mut conn: Connection = Connection::open(db_path)?;
    println!("Connected to Database");

    // NOTE: Creating a table using query
    match conn.execute(
        "CREATE TABLE IF NOT EXISTS notes (
            id INTEGER PRIMARY KEY,
            title TEXT NOT NULL,
            content TEXT NOT NULL,
            created_at TEXT NOT NULL DEFAULT (datetime('now'))
        )",
        [],
    ) {
        Ok(_) => println!("Notes table created"),
        Err(e) => {
            println!("Failed to create table : {}", e);
            std::process::exit(0);
        }
    }

    Ok(conn)
}
