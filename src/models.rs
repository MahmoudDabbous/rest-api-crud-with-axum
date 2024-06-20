use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct NoteRow {
    pub id: i32,
    pub title: Option<String>,
    pub content: Option<String>,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct CreateNote {
    pub title: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct UpdateNote {
    pub title: Option<String>,
    pub content: Option<String>,
}
