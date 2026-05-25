use std::error::Error;

use async_trait::async_trait;

use crate::models::{SignInDto, SignInError, SignInResult};

#[async_trait]
pub trait Handler: Sync + Send {
    async fn sign_in(&self, dto: SignInDto) -> Result<SignInResult, SignInError>;
    async fn purge_refresh_tokens(&self) -> Result<(), Box<dyn Error>>;
}
