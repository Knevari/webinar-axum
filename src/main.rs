mod db;
mod rest;
mod view;

use std::net::SocketAddr;

use crate::db::init_db;
use anyhow::Result;
use axum::{Extension, Router};
use sqlx::SqlitePool;

fn router(connection_pool: SqlitePool) -> Router {
    Router::new()
        .nest_service("/books", rest::books_service())
        .nest_service("/", view::view_service())
        .layer(Extension(connection_pool))
}

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables from .env if available
    dotenv::dotenv().ok();

    // Initialize the database and obtain a connection pool
    let connection_pool = init_db().await?;

    let app = router(connection_pool);
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
