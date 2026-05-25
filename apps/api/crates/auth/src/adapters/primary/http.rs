use std::sync::Arc;

use crate::{
    api::{SignInRequest, SignInResponse},
    models::SignInError,
    ports::Handler,
};
use axum::{Json, Router, extract::State, http::StatusCode, routing::post};
use axum_valid::Valid;

type RouteState = Arc<dyn Handler>;

pub fn create(core: Arc<dyn Handler>) -> Router {
    Router::new()
        .route("/sign-in", post(sign_in))
        .with_state(core)
}

async fn sign_in(
    State(core): State<RouteState>,
    Valid(Json(payload)): Valid<Json<SignInRequest>>,
) -> Result<Json<SignInResponse>, StatusCode> {
    let result = core.sign_in(payload.into()).await;
    match result {
        Ok(result) => Ok(Json(result.into())),
        Err(err) => match err {
            SignInError::IncorrectPassword => Err(StatusCode::UNAUTHORIZED),
            SignInError::UserNotFound => Err(StatusCode::NOT_FOUND),
            SignInError::InternalServerError => Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
    }
}
