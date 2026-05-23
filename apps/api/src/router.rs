use std::{sync::Arc, time::Duration};

use axum::{Router, http::StatusCode, routing::get};
use sqlx::{Pool, Postgres};
use tower_http::{
    cors::{Any, CorsLayer},
    timeout::TimeoutLayer,
};

use crate::{auth, card, config::Config, cron::CronScheduler, user};

pub fn create_router(
    config: &Config,
    db_pool: Pool<Postgres>,
    cron_manager: Arc<CronScheduler>,
) -> Router {
    let (user_http_adapter, user_service) = user::register(db_pool.clone());
    let user_service = Arc::new(user_service);
    let auth_http_adapter = auth::register(
        config.auth.clone(),
        db_pool.clone(),
        cron_manager.clone(),
        user_service,
    );
    let card_http_adapter = card::register(config.card.clone());

    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .nest("/auth", auth_http_adapter)
        .nest("/users", user_http_adapter)
        .nest("/cards", card_http_adapter)
        .layer((
            // Graceful shutdown will wait for outstanding requests to complete. Add a timeout so
            // requests don't hang forever.
            TimeoutLayer::with_status_code(StatusCode::REQUEST_TIMEOUT, Duration::from_secs(10)),
            CorsLayer::new().allow_origin(Any),
        ))
}
