use std::convert::Infallible;

use axum::http::Request;
use tonic::{
    body::Body,
    server::NamedService,
    transport::{Server, server::Router},
};
use tower::Service;

use crate::config::Config;

pub fn create_grpc<S>(config: &Config, user: S) -> Router
where
    S: Service<Request<Body>, Error = Infallible> + NamedService + Clone + Send + Sync + 'static,
    S::Response: axum::response::IntoResponse,
    S::Future: Send + 'static,
{
    let router = Server::builder().add_service(user);

    if !config.is_production {
        let reflection_service = tonic_reflection::server::Builder::configure()
            .register_encoded_file_descriptor_set(api::FILE_DESCRIPTOR_SET)
            .build_v1()
            .expect("failed to build gRPC v1 reflection service");

        router.add_service(reflection_service)
    } else {
        router
    }
}
