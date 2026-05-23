mod adapters;
mod config;
mod core;
mod models;
mod ports;

use std::sync::Arc;

use axum::Router;
pub use config::Config;
use sqlx::{Pool, Postgres};

use crate::{
    auth::adapters::secondary::{PostgresAdapter, UserAdapter},
    pkg::cron::Scheduler,
    user,
};

pub fn register(
    config: Config,
    postgres: Pool<Postgres>,
    cron: Arc<dyn Scheduler>,
    user_service: Arc<user::Service>,
) -> Router {
    let user_adapter = Arc::new(UserAdapter::new(user_service));
    let postgres_adapter = Arc::new(PostgresAdapter::new(postgres));
    let core = Arc::new(core::Core::new(
        config.secret,
        postgres_adapter,
        user_adapter,
    ));

    adapters::primary::CronAdapter::new(cron, core.clone());
    adapters::primary::http(core.clone())
}
