use std::sync::Arc;

use axum::{
    Router,
    extract::{Json, State},
    routing::post,
};
use http::StatusCode;

use crate::{
    adapters::primary::dto::SignInDto,
    api::SignInResponse,
    models::{self, SignInError},
    ports::Handler,
};

#[derive(Clone)]
pub struct HttpAdapter;

impl HttpAdapter {
    pub fn new(core: Arc<dyn Handler>) -> Router {
        Router::new()
            .route("/sign-in", post(sign_in))
            .with_state(core)
    }
}

async fn sign_in(
    State(core): State<Arc<dyn Handler>>,
    Json(payload): Json<SignInDto>,
) -> Result<Json<SignInResponse>, StatusCode> {
    let result = core
        .sign_in(models::SignInDto {
            email: payload.email,
            password: payload.password,
        })
        .await;
    match result {
        Ok(result) => Ok(Json(result.into())),
        Err(err) => match err {
            SignInError::IncorrectPassword => Err(StatusCode::UNAUTHORIZED),
            SignInError::UserNotFound => Err(StatusCode::UNAUTHORIZED),
            SignInError::InternalServerError => Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
    }
}
