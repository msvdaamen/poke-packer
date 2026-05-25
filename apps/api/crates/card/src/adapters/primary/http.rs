use std::sync::Arc;

use axum::{Router, extract::State, response::Json, routing::get};

use crate::{models::Card, ports::Handler};

pub fn create(core: Arc<dyn Handler>) -> Router {
    Router::new().route("/", get(handler)).with_state(core)
}

async fn handler(State(core): State<Arc<dyn Handler>>) -> Json<Vec<Card>> {
    Json(core.get_cards().await)
}
