use std::sync::Arc;

use axum::Router;

mod adapters;
mod config;
mod core;
mod models;
mod ports;

pub use config::Config;

pub fn register(_config: Config) -> Router {
    let storage = Box::new(adapters::secondary::StorageAdapter::new());
    let core = Arc::new(core::Core::new(storage));
    adapters::primary::http(core.clone())
}
