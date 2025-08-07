use std::env;

use dotenv::dotenv;

use crate::{DEFAULT_PASS, DEFAULT_USER, HARBOR_HOST, client::error::ClientError};

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

impl Default for HarborClient {
    fn default() -> Self {
        Self::from_env(DEFAULT_USER, DEFAULT_PASS).unwrap()
    }
}

impl HarborClient {
    pub fn from_env(u: &str, p: &str) -> Result<Self, ClientError> {
        dotenv().ok();
        HarborClient::new(env::var(HARBOR_HOST)?, env::var(u)?, env::var(p)?)
    }

    pub fn new(base_url: String, username: String, password: String) -> Result<Self, ClientError> {
        let client = HarborClient {
            base_url,
            username,
            password,
            client: reqwest::Client::builder()
                //.add_root_certificate(cert)
                // TODO!: Figure out certification
                .danger_accept_invalid_certs(true)
                .build()?,
        };

        Ok(client)
    }
}
