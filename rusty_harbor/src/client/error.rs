use reqwest::StatusCode;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("not the expected format: {0}")]
    Deserialize(#[from] serde_json::Error),

    #[error("missing or invalid environment variable: {0}")]
    Environment(#[from] std::env::VarError),

    #[error("error creating the client: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("error response ({status}): {message}")]
    Response { status: StatusCode, message: String },
}
