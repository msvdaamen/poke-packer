use std::sync::Arc;

use ::api::grpc::user::user_service_server::UserServiceServer;
use axum::Router;
use sqlx::{Pool, Postgres};

use crate::adapters::primary::GrpcAdapter;

pub use self::adapters::primary::Service;

mod adapters;
pub mod api;
mod core;
mod models;
mod ports;

pub fn register(pool: Pool<Postgres>) -> (Router, Service, UserServiceServer<GrpcAdapter>) {
    let storage = Box::new(adapters::secondary::PostgresAdapter::new(pool));

    let core = Arc::new(core::Core::new(storage));

    let http_adapter = adapters::primary::http(core.clone());
    let service = Service::new(core.clone());
    let grpc_adapter = UserServiceServer::new(GrpcAdapter::new(core.clone()));

    (http_adapter, service, grpc_adapter)
}
