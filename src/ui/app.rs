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
}

impl App {
    pub fn new() -> App {
        App {
            note_title: String::new(),
            note_content: String::new(),
            save_note: false,
            current_screen: CurrentScreen::CreateNote,
            currently_editing: None,
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
}
