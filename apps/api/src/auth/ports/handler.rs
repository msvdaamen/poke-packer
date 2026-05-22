use async_trait::async_trait;

use crate::auth::models::{SignInDto, SignInError, SignInResult};

#[async_trait]
pub trait Handler: Sync + Send {
    async fn sign_in(&self, dto: SignInDto) -> Result<SignInResult, SignInError>;
}
