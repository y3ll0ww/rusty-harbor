use derive_builder::Builder;
use derive_harbor::Harbor;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde::Serialize;

use crate::{request::HarborRequest, response::v2::search::Search};

/// The Search endpoint returns information about the projects and repositories offered at public
/// status or related to the current logged in user. The response includes the project and
/// repository list in a proper display order.
#[derive(Builder, Harbor, Serialize)]
#[harbor(
    url = "health",
    response = Search,
)]
pub struct GetSearch {
    /// An unique ID for the request.
    #[builder(default)]
    #[header(rename = "X-Request-Id")]
    pub request_id: Option<String>,
    /// Search parameter for project and repository name.
    #[builder(default)]
    #[serde(rename = "q")]
    pub query: String,
}

impl GetSearch {
    pub fn builder(query: &str) -> GetSearchBuilder {
        GetSearchBuilder::default()
            .query(query.to_string())
            .to_owned()
    }
}
