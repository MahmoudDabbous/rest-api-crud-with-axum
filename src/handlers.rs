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
    let notes = sqlx::query_as!(
        NoteRow,
        "
            SELECT id, title, content
            FROM notes
        "
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((StatusCode::OK, Json(notes)))
}

pub async fn create_note(
    state: State<Arc<AppState>>,
    Json(payload): Json<CreateNote>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let result = sqlx::query_as!(
        NoteRow,
        "
            INSERT INTO notes (title, content)
            VALUES (?, ?)
        ",
        payload.title,
        payload.content
    )
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
    let note = sqlx::query_as!(
        NoteRow,
        "
            SELECT id, title, content
            FROM notes
            WHERE id = ?
        ",
        id
    )
    .fetch_one(&state.db)
    .await
    .map_err(|_| (StatusCode::NOT_FOUND, "Note not found".to_string()))?;

    Ok((StatusCode::OK, Json(note)))
}

pub async fn update_note(
    Path(id): Path<i32>,
    state: State<Arc<AppState>>,
    Json(payload): Json<UpdateNote>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let result = sqlx::query_as!(
        NoteRow,
        "
            UPDATE notes
            SET title = ?, content = ?
            WHERE id = ?
        ",
        payload.title,
        payload.content,
        id
    )
    .execute(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((StatusCode::OK, Json(result.rows_affected())))
}
pub async fn delete_note(
    Path(id): Path<i32>,
    state: State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    sqlx::query_as!(
        NoteRow,
        "
            DELETE FROM notes
            WHERE id = ?
        ",
        id
    )
    .execute(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((StatusCode::NO_CONTENT, "".to_string()))
}
