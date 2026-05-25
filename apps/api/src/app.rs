use std::sync::Arc;

use axum::Router;
use sqlx::{Pool, Postgres};

use crate::{config::Config, cron::CronScheduler, router::create_router};

pub fn create_app(
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

    create_router(auth_http_adapter, user_http_adapter, card_http_adapter)
}
