use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SignInDto {
    pub email: String,
    pub password: String,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignInResponse {
    pub access_token: String,
    pub refresh_token: String,
}

pub enum SignInError {
    UserNotFound,
    IncorrectPassword,
    InternalServerError,
}
