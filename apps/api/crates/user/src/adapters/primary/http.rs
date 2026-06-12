use std::sync::Arc;

use axum::Router;

use crate::ports::Handler;

#[derive(Clone)]
pub struct HttpAdapter;

impl HttpAdapter {
    pub fn new(core: Arc<dyn Handler>) -> Router {
        Router::new().with_state(core)
    }
}
