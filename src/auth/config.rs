use std::env;

use crate::config::FromEnv;

#[derive(Debug, Clone)]
pub struct Config {
    secret: String,
}

impl FromEnv for Config {
    fn from_env() -> Result<Self, String> {
        Ok(Self {
            secret: env::var("AUTH_SECRET_KEY").map_err(|_| "AUTH_SECRET_KEY is not set")?,
        })
    }
}
