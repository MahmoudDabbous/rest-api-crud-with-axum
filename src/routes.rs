use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::{handlers, AppState};

pub fn notes_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/notes", get(handlers::get_notes))
        .route("/notes", post(handlers::create_note))
        .route(
            "/notes/:id",
            get(handlers::get_note_by_id)
                .put(handlers::update_note)
                .delete(handlers::delete_note),
        )
        .with_state(app_state)
}
