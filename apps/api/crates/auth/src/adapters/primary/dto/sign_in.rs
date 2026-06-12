use serde::Deserialize;
use shared::types::Email;

#[derive(Debug, Deserialize)]
pub struct SignInDto {
    pub email: Email,

    pub password: String,
}
