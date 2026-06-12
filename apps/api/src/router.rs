use std::time::Duration;

use axum::{Router, http::StatusCode, routing::get};
use tower_http::{
    cors::{Any, CorsLayer},
    timeout::TimeoutLayer,
};

pub fn create_router(auth_adapter: Router, user_adapter: Router, card_adapter: Router) -> Router {
    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .merge(auth_adapter)
        .merge(user_adapter)
        .merge(card_adapter)
        .layer((
            TimeoutLayer::with_status_code(StatusCode::REQUEST_TIMEOUT, Duration::from_secs(10)),
            CorsLayer::new().allow_origin(Any),
        ))
}
