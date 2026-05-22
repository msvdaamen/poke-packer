mod adapters;
mod config;
mod core;
mod models;
mod ports;

use std::sync::Arc;

use axum::Router;
pub use config::Config;

use crate::{auth::adapters::secondary::UserAdapter, user};

pub fn register(config: Config, user_service: Arc<user::Service>) -> Router {
    let user_adapter = Arc::new(UserAdapter::new(user_service));
    let core = Arc::new(core::Core::new(config.secret, user_adapter));

    adapters::primary::http(core)
}
