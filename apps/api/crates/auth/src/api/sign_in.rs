use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::models;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SignInResponse {
    pub access_token: String,
    pub refresh_token: String,
}

impl From<models::SignInResult> for SignInResponse {
    fn from(value: models::SignInResult) -> Self {
        Self {
            access_token: value.access_token,
            refresh_token: value.refresh_token,
        }
    }
}

#[derive(Debug, Validate, Deserialize)]
pub struct SignInRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
}

impl Into<models::SignInDto> for SignInRequest {
    fn into(self: SignInRequest) -> models::SignInDto {
        models::SignInDto {
            email: self.email,
            password: self.password,
        }
    }
}
