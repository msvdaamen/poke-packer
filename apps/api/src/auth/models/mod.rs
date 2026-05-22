mod sign_in_dto;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub use sign_in_dto::*;

pub struct UserWithPassword {
    pub id: Uuid,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessTokenClaims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
}
