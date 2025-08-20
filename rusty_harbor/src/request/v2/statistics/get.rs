use derive_builder::Builder;
use derive_harbor::Harbor;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde::Serialize;

use crate::{request::HarborRequest, response::v2::statistics::Statistic};

/// The Search endpoint returns information about the projects and repositories offered at public
/// status or related to the current logged in user. The response includes the project and
/// repository list in a proper display order.
#[derive(Builder, Harbor, Serialize)]
#[harbor(
    url = "statistics",
    response = Statistic,
)]
pub struct GetStatistics {
    /// An unique ID for the request.
    #[builder(default)]
    #[header(rename = "X-Request-Id")]
    pub request_id: Option<String>,
}

impl GetStatistics {
    pub fn builder() -> GetStatisticsBuilder {
        GetStatisticsBuilder::default().to_owned()
    }
}
