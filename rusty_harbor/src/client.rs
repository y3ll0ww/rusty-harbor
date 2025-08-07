use std::env;

use dotenv::dotenv;

use crate::{client::error::ClientError, DEFAULT_PASS, DEFAULT_USER, HARBOR_HOST};

mod dispatch;
mod error;
#[cfg(test)]
mod tests;

/// Robot account
pub struct HarborClient {
    base_url: String,
    username: String,
    password: String,
    client: reqwest::Client,
}

impl HarborClient {
    pub fn default() -> Result<Self, ClientError> {
        Self::from_env(DEFAULT_USER, DEFAULT_PASS)
    }

    pub fn from_env(u: &str, p: &str) -> Result<Self, ClientError> {
        dotenv().ok();

        let base_url = env::var(HARBOR_HOST)?;
        let username = env::var(u)?;
        let password = env::var(p)?;

        HarborClient::new(base_url, username, password)
    }

    pub fn new(base_url: String, username: String, password: String) -> Result<Self, ClientError> {
        let client = HarborClient {
            base_url,
            username,
            password,
            client: reqwest::Client::builder()
                //.add_root_certificate(cert)
                // TODO!: Figure out certification for K3s
                .danger_accept_invalid_certs(true)
                .build()?,
        };

        Ok(client)
    }
}
