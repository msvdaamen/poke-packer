use std::time::Duration;

use axum::{Router, http::StatusCode, routing::get};
use tower_http::{
    cors::{Any, CorsLayer},
    timeout::TimeoutLayer,
};

pub fn create_router(auth_router: Router, user_router: Router, card_router: Router) -> Router {
    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .nest("/auth", auth_router)
        .nest("/users", user_router)
        .nest("/cards", card_router)
        .layer((
            TimeoutLayer::with_status_code(StatusCode::REQUEST_TIMEOUT, Duration::from_secs(10)),
            CorsLayer::new().allow_origin(Any),
        ))
}
