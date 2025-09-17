use derive_builder::Builder;
use derive_harbor::Harbor;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde::Serialize;

use crate::{request::HarborRequest, response::v2::health::OverallHealthStatus};

/// Check the status of Harbor components. This path does not require authentication.
#[derive(Builder, Harbor, Serialize)]
#[harbor(
    url = "health",
    response = OverallHealthStatus,
)]
pub struct GetHealth {
    /// An unique ID for the request.
    #[builder(default)]
    #[header(rename = "X-Request-Id")]
    pub request_id: Option<String>,
}

impl GetHealth {
    pub fn builder() -> GetHealthBuilder {
        GetHealthBuilder::default().to_owned()
    }
}
