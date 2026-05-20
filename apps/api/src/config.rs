use crate::{auth, card};
use dotenv::dotenv;
use std::env;

pub trait FromEnv: Sized {
    fn from_env() -> Result<Self, String>;
}

#[derive(Debug, Clone)]
pub struct Config {
    pub auth: auth::Config,
    pub card: card::Config,
}

impl FromEnv for Config {
    fn from_env() -> Result<Self, String> {
        dotenv().ok();

        let auth = auth::Config::from_env()?;
        let card = card::Config::from_env()?;

        Ok(Self { auth, card })
    }
}
