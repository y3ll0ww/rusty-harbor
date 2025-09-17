use derive_builder::Builder;
use derive_harbor::Harbor;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde::Serialize;

use crate::{request::HarborRequest, response::v2::configure::InternalConfigurationsResponse};

/// This endpoint is for retrieving system configurations that only provides for internal api call.
#[derive(Builder, Harbor, Serialize)]
#[harbor(
    url = "internalconfig",
    response = InternalConfigurationsResponse,
)]
pub struct GetConfiguration {
    /// An unique ID for the request.
    #[builder(default)]
    #[header(rename = "X-Request-Id")]
    pub request_id: Option<String>,
}

impl GetConfiguration {
    pub fn builder() -> GetConfigurationBuilder {
        GetConfigurationBuilder::default().to_owned()
    }
}
