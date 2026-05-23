use std::error::Error;

use async_trait::async_trait;

use crate::auth::models::{SignInDto, SignInError, SignInResponse};

#[async_trait]
pub trait Handler: Sync + Send {
    async fn sign_in(&self, dto: SignInDto) -> Result<SignInResponse, SignInError>;
    async fn purge_refresh_tokens(&self) -> Result<(), Box<dyn Error>>;
}
