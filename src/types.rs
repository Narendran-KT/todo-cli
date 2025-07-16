use chrono::NaiveDateTime;

pub struct Note {
    pub id: u32,
    pub title: String,
    pub content: String,
    pub created_at: NaiveDateTime,
}
