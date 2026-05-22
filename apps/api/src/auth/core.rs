use std::{
    ops::Add,
    sync::Arc,
};

use argon2::{Argon2, PasswordHash, PasswordVerifier};
use async_trait::async_trait;
use chrono::Duration;
use jsonwebtoken::{
    EncodingKey, Header,
    jws::{Jws, encode},
};
use serde::Serialize;

use crate::auth::{
    models::{AccessTokenClaims, SignInDto, SignInError, SignInResult},
    ports::{self},
};

pub struct Core {
    secret: String,
    user: Arc<dyn ports::User>,
}

impl Core {
    pub fn new(secret: String, user: Arc<dyn ports::User>) -> Self {
        Self { secret, user }
    }
}

#[async_trait]
impl ports::Handler for Core {
    async fn sign_in(&self, dto: SignInDto) -> Result<SignInResult, SignInError> {
        let user = self.user.find_with_password_by_email(&dto.email).await;
        let Some(user) = user else {
            return Err(SignInError::UserNotFound);
        };
        let parsed_hash =
            PasswordHash::new(&user.password).map_err(|_| SignInError::InternalServerError)?;
        Argon2::default()
            .verify_password(&dto.password.into_bytes(), &parsed_hash)
            .map_err(|_| SignInError::IncorrectPassword)?;
        let claims = AccessTokenClaims {
            sub: user.id.to_string(),
            exp: chrono::Utc::now().add(Duration::minutes(5)).timestamp() as usize,
            iat: chrono::Utc::now().timestamp() as usize,
        };
        let token =
            create_token(&self.secret, &claims).map_err(|_| SignInError::InternalServerError)?;

        Ok(SignInResult {
            access_token: jws_to_string(token),
        })
    }
}

fn jws_to_string<T>(jws: Jws<T>) -> String {
    format!("{}.{}.{}", jws.protected, jws.payload, jws.signature)
}

fn create_token<T: Serialize>(
    secret: &str,
    claims: &T,
) -> Result<Jws<T>, jsonwebtoken::errors::Error> {
    encode(
        &Header::default(),
        Some(claims),
        &EncodingKey::from_secret(secret.as_ref()),
    )
}
