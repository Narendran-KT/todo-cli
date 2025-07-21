use todo::db::list_all_notes;
use todo::types::Note;

// #[deive(Debug)]
pub enum CurrentScreen {
    CreateNote,
    ListNotes,
    Editing,
    Exiting,
}

pub enum CurrentlyEditing {
    Title,
    Content,
}

pub struct App {
    pub note_title: String,
    pub note_content: String,
    pub save_note: bool,
    pub current_screen: CurrentScreen,
    pub currently_editing: Option<CurrentlyEditing>,
    pub currently_selected: Option<usize>,
    pub total_notes_count: usize,
    pub notes_list: Vec<Note>,
}

impl App {
    pub fn new() -> App {
        let notes = match list_all_notes() {
            Ok(note) => note,
            Err(_) => Vec::new(),
        };
        let count = notes.len();
        let selection: Option<usize> = if count > 0 { Some(0) } else { None };

        App {
            note_title: String::new(),
            note_content: String::new(),
            save_note: false,
            current_screen: CurrentScreen::CreateNote,
            currently_editing: None,
            currently_selected: selection,
            total_notes_count: count,
            notes_list: notes,
        }
    }

    pub fn save_note(&mut self) {
        todo!();
    }

    pub fn toggle_editing(&mut self) {
        if let Some(edit_mode) = &self.currently_editing {
            todo!();
        }
    }

    pub fn next_note_row(&mut self) {
        if self.total_notes_count <= 0 {
            self.currently_selected = None;
            return;
        };

        self.currently_selected = Some(match self.currently_selected {
            Some(i) if i + 1 < self.total_notes_count => i + 1,
            _ => 0,
        })
    }

    pub fn prev_note_row(&mut self) {
        if self.total_notes_count <= 0 {
            self.currently_selected = None;
            return;
        };

        self.currently_selected = Some(match self.currently_selected {
            Some(i) if i > 0 => i - 1,
            Some(_) => self.total_notes_count - 1,
            _ => 0,
        })
    }
}
