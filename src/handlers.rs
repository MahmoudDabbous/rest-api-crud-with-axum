use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::sync::Arc;

use crate::{
    models::{CreateNote, NoteRow, UpdateNote},
    AppState,
};

pub async fn get_notes(
    state: State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let notes = sqlx::query_as::<_, NoteRow>("SELECT * FROM notes")
        .fetch_all(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok((StatusCode::OK, Json(notes)))
}

pub async fn create_note(
    state: State<Arc<AppState>>,
    Json(payload): Json<CreateNote>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let result = sqlx::query("INSERT INTO notes (title, content) VALUES (?, ?)")
        .bind(&payload.title)
        .bind(&payload.content)
        .execute(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((
        StatusCode::CREATED,
        format!("Note created with ID: {}", result.last_insert_id()),
    ))
}

pub async fn get_note_by_id(
    Path(id): Path<i32>,
    state: State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let note = sqlx::query_as::<_, NoteRow>("SELECT * FROM notes WHERE id = ?")
        .bind(id)
        .fetch_one(&state.db)
        .await
        .map_err(|e| (StatusCode::NOT_FOUND, e.to_string()))?;

    Ok((StatusCode::OK, Json(note)))
}

pub async fn update_note(
    Path(id): Path<i32>,
    state: State<Arc<AppState>>,
    Json(payload): Json<UpdateNote>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let row = sqlx::query_as::<_, NoteRow>("SELECT * FROM notes WHERE id = ?")
        .bind(id)
        .fetch_one(&state.db)
        .await
        .map_err(|_| (StatusCode::NOT_FOUND, "Note not found".to_string()))?;

    sqlx::query("UPDATE notes SET title = ?, content = ? WHERE id = ?")
        .bind(&payload.title.unwrap_or(row.title.unwrap()))
        .bind(&payload.content.unwrap_or(row.content.unwrap()))
        .bind(id)
        .execute(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((StatusCode::OK, "Note updated".to_string()))
}
pub async fn delete_note(
    Path(id): Path<i32>,
    state: State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let result = sqlx::query("DELETE FROM notes WHERE id = ?")
        .bind(id)
        .execute(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if result.rows_affected() == 0 {
        return Err((StatusCode::NOT_FOUND, "Note not found".to_string()));
    }

    Ok((StatusCode::NO_CONTENT, "".to_string()))
}
