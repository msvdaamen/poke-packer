use std::sync::Arc;

use crate::auth::{
    models::{SignInDto, SignInResponse},
    ports::Handler,
};
use axum::{Json, Router, extract::State, http::StatusCode, routing::post};

type RouteState = Arc<dyn Handler>;

pub fn create(core: Arc<dyn Handler>) -> Router {
    Router::new()
        .route("/sign-in", post(sign_in))
        .with_state(core)
}

async fn sign_in(
    State(core): State<RouteState>,
    Json(payload): Json<SignInDto>,
) -> Result<Json<SignInResponse>, StatusCode> {
    let result = core.sign_in(payload).await;
    match result {
        Ok(result) => Ok(Json(result)),
        Err(err) => match err {
            crate::auth::models::SignInError::IncorrectPassword => Err(StatusCode::UNAUTHORIZED),
            crate::auth::models::SignInError::UserNotFound => Err(StatusCode::NOT_FOUND),
            crate::auth::models::SignInError::InternalServerError => {
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        },
    }
}
