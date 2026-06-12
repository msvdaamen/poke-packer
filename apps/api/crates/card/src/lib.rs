use std::sync::Arc;

mod adapters;
mod config;
mod core;
mod models;
mod ports;

pub use adapters::primary::HttpAdapter;
use axum::Router;
pub use config::Config;

pub fn register(_config: Config) -> Router {
    let storage = Box::new(adapters::secondary::StorageAdapter::new());
    let core = Arc::new(core::Core::new(storage));
    HttpAdapter::new(core.clone())
}
