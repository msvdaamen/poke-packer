use std::sync::Arc;

use axum::Router as AxumRouter;
use tonic::transport::server::Router as GrpcRouter;

use sqlx::{Pool, Postgres};

use crate::{config::Config, cron::CronScheduler, grpc::create_grpc, router::create_router};

pub fn create_app(
    config: &Config,
    db_pool: Pool<Postgres>,
    cron_manager: Arc<CronScheduler>,
) -> (AxumRouter, GrpcRouter) {
    let (user_http, user_grpc) = user::register(db_pool.clone());
    let auth_http = auth::register(config.auth.clone(), db_pool.clone(), cron_manager.clone());
    let card_http = card::register(config.card.clone());

    (
        create_router(auth_http, user_http, card_http),
        create_grpc(&config, user_grpc),
    )
}
