use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("missing or invalid environment variable")]
    Environment(#[from] std::env::VarError),
    #[error("error creating the client")]
    Reqwest(#[from] reqwest::Error),
}