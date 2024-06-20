use std::sync::Arc;

use dotenvy::dotenv;
use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};

mod handlers;
mod models;
mod routes;

pub struct AppState {
    db: Pool<MySql>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DB_URL must be set");

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");
    let app_state = Arc::new(AppState { db: pool });

    let app = routes::notes_router(app_state);

    let listener = tokio::net::TcpListener::bind("localhost:8000")
        .await
        .unwrap();

    println!("Listening on: {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
