use std::sync::Arc;

use axum::Router;

mod adapters;
mod config;
mod core;
mod models;
mod ports;

pub use config::Config;

pub fn register(config: Config) -> Router {
    let storage = Arc::new(adapters::secondary::StorageAdapter::new());
    // let core = Arc::new(core::Core::new(storage));
    let core2 = Arc::new(core::Core::new(storage.clone()));
    // If you add more primary adapters, just `core.clone()` for each.
    adapters::primary::http(core2.clone())
}
