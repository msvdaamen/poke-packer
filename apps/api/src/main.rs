mod auth;
mod card;
mod config;
mod user;

use std::sync::Arc;

use axum::{Router, routing::get};
use sqlx::postgres::PgPoolOptions;

use crate::config::FromEnv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::Config::from_env()?;

    let db_pool = PgPoolOptions::new().connect(&config.database_url).await?;

    let (user_http_adapter, user_service) = user::register(db_pool);
    let user_service = Arc::new(user_service);
    let auth_http_adapter = auth::register(config.auth, user_service);
    let card_http_adapter = card::register(config.card);

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .nest("/auth", auth_http_adapter)
        .nest("/users", user_http_adapter)
        .nest("/cards", card_http_adapter);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!(
        "Server running on: http://{}",
        listener.local_addr().unwrap().to_string()
    );
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
