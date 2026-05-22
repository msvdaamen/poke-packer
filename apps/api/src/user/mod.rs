use std::sync::Arc;

use axum::Router;
use sqlx::{Pool, Postgres};

pub use self::adapters::primary::Service;

mod adapters;
mod core;
mod models;
mod ports;

pub fn register(pool: Pool<Postgres>) -> (Router, Service) {
    let storage = Arc::new(adapters::secondary::PostgresAdapter::new(pool));
    // let core = Arc::new(core::Core::new(storage));
    let core2 = Arc::new(core::Core::new(storage.clone()));
    // If you add more primary adapters, just `core.clone()` for each.
    let http_adapter = adapters::primary::http(core2.clone());
    let service = Service::new(core2.clone());

    (http_adapter, service)
}
