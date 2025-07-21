use crate::types::Note;
use chrono::NaiveDateTime;
use once_cell::sync::Lazy;
use rusqlite::{Connection, Result, params};
use std::sync::Mutex;

static DB_CONNECTION: Lazy<Mutex<Connection>> = Lazy::new(|| {
    let conn = connect_db().expect("Failed to connect to database");
    Mutex::new(conn)
});

fn get_db_path() -> std::path::PathBuf {
    let mut path: std::path::PathBuf =
        dirs::data_dir().unwrap_or_else(|| std::path::PathBuf::from("."));
    path.push("notes-cli");
    // eprintln!("Your notes will be saving to {}", path.display());
    std::fs::create_dir_all(&path).unwrap();
    path.push("notes.db");
    path
}

pub fn connect_db() -> Result<Connection> {
    let db_path = get_db_path();
    let conn: Connection = Connection::open(db_path)?;
    // eprintln!("Connected to Database");

    // NOTE: Creating a table using query
    match conn.execute(
        "CREATE TABLE IF NOT EXISTS notes (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            content TEXT NOT NULL,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            updated_at TEXT NOT NULL DEFAULT (datetime('now'))
        )",
        [],
    ) {
        Ok(_) => {
            // eprintln!("Notes table created")
        }
        Err(e) => {
            // eprintln!("Failed to create table : {}", e);
            std::process::exit(0);
        }
    }
    Ok(conn)
}

pub fn list_all_notes() -> Result<Vec<Note>> {
    let conn = DB_CONNECTION.lock().unwrap();
    let query = "SELECT * FROM notes";
    let mut query_data = conn.prepare(query)?;

    let note_iter = query_data.query_map([], |row| {
        let created_at_str: String = row.get(3)?;
        let created_at = NaiveDateTime::parse_from_str(&created_at_str, "%Y-%m-%d %H:%M:%S")
            .map_err(|e| {
                rusqlite::Error::FromSqlConversionFailure(
                    0,
                    rusqlite::types::Type::Text,
                    Box::new(e),
                )
            })?;
        Ok(Note {
            id: row.get(0)?,
            title: row.get(1)?,
            content: row.get(2)?,
            created_at,
        })
    })?;

    let notes: Vec<Note> = note_iter.collect::<rusqlite::Result<Vec<Note>>>()?;

    Ok(notes)
}

pub fn insert_note(title: &String, content: &String) {
    let conn = DB_CONNECTION.lock().unwrap();
    let query = "INSERT INTO notes (title, content) VALUES (?1, ?2)";
    conn.execute(query, params![title, content]).or_else(|e| {
        // eprintln!("Error : {}", e);
        std::process::exit(0);
        Err(e)
    });
}

pub fn update_note(title: &String, content: &String, id: &u32) {
    let conn = DB_CONNECTION.lock().unwrap();
    let query =
        "UPDATE notes SET title = ?1, content = ?2, updated_at = datetime('now') WHERE id = ?3";

    conn.execute(query, params![title, content, id])
        .or_else(|e| {
            // eprintln!("Error : {}", e);
            std::process::exit(0);
            Err(e)
        });
}

struct Dummy_Notes {
    title: String,
    content: String,
}

pub fn dummy_notes_to_test() {
    // let mut list: Vec<Dummy_Notes> = Vec::new();

    for i in 0..5 {
        let note: Dummy_Notes = Dummy_Notes {
            title: String::from(format!("Note {}", i + 1)),
            content: String::from(format!("This is note {} content", i + 1)),
        };
        // list.push(note);

        insert_note(&note.title, &note.content);
    }

    // std::thread::sleep(std::time::Duration::from_secs(10));
    //
    // let conn = DB_CONNECTION.lock().unwrap();
    // let query = "DELETE FROM notes";
    // match conn.execute(query, params![]) {
    //     Ok(_) => std::process::exit(0),
    //     Err(_) => std::process::exit(0),
    // }
}
