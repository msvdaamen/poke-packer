use std::sync::Arc;

use axum::{
    Router,
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::get,
};
use uuid::Uuid;

use crate::user::{models::User, ports::Handler};

pub fn create(core: Arc<dyn Handler>) -> Router {
    Router::new().route("/{id}", get(handler)).with_state(core)
}

async fn handler(
    Path(user_id): Path<Uuid>,
    State(core): State<Arc<dyn Handler>>,
) -> Result<Json<User>, StatusCode> {
    if let Some(user) = core.get_by_id(user_id).await {
        return Ok(Json(user));
    }
    Err(StatusCode::NOT_FOUND)
}
