use std::sync::Arc;

use axum::Router;
use sqlx::{Pool, Postgres};

pub use self::adapters::primary::Service;

mod adapters;
pub mod api;
mod core;
mod models;
mod ports;

pub fn register(pool: Pool<Postgres>) -> (Router, Service) {
    let storage = Box::new(adapters::secondary::PostgresAdapter::new(pool));
    let core = Arc::new(core::Core::new(storage));
    let http_adapter = adapters::primary::http(core.clone());
    let service = Service::new(core.clone());

    (http_adapter, service)
}
