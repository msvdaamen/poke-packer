mod adapters;
pub mod api;
mod config;
mod core;
mod models;
mod ports;

use std::sync::Arc;

pub use adapters::primary::HttpAdapter;
use axum::Router;
pub use config::Config;
use sqlx::{Pool, Postgres};

use crate::adapters::secondary::{PostgresAdapter, UserAdapter};
use shared::cron::Scheduler;

pub fn register(config: Config, postgres: Pool<Postgres>, cron: Arc<dyn Scheduler>) -> Router {
    let user_adapter = Box::new(UserAdapter::new(&config.grpc_url.clone()));
    let postgres_adapter = Box::new(PostgresAdapter::new(postgres));
    let core = Arc::new(core::Core::new(
        config.secret,
        postgres_adapter,
        user_adapter,
    ));

    adapters::primary::CronAdapter::new(cron, core.clone());
    adapters::primary::HttpAdapter::new(core.clone())
}
