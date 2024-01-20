use serde::Deserialize;
use std::env;
pub struct Envs {
    pub client_id: String,
    pub client_secret: String,
}

pub struct App {}
pub struct Events {}

#[derive(Deserialize)]
pub struct Query {
    pub code: String,
}

impl Envs {
    pub fn new() -> Option<Self> {
        let client_id: String = if let Ok(val) = env::var("CLIENT_ID") {
            val
        } else {
            return None;
        };

        let client_secret: String = if let Ok(val) = env::var("CLIENT_SECRET") {
            val
        } else {
            return None;
        };

        Some(Self {
            client_id,
            client_secret,
        })
    }
}
