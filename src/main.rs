mod auth;
mod card;
mod config;

use axum::{Router, routing::get};

use crate::config::FromEnv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::Config::from_env()?;
    let card_http_adapter = card::register(config.card);

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .nest("/cards", card_http_adapter);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!(
        "Server running on: http://{}",
        listener.local_addr().unwrap().to_string()
    );
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
