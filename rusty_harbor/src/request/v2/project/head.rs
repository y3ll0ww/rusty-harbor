use derive_builder::Builder;
use derive_harbor::Harbor;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde::Serialize;
use serde_json::Value;

use crate::request::HarborRequest;

/// Get the deletable status of the project.
#[derive(Builder, Harbor, Serialize)]
#[harbor(
    url = "projects",
    response = Value,
)]
pub struct HeadProjects {
    /// An unique ID for the request
    #[header]
    #[builder(default)]
    #[serde(rename = "X-Request-Id")]
    pub id: Option<String>,
    /// Project name for checking exists.
    #[header]
    pub project_name: String,
}

impl HeadProjects {
    pub fn builder(project_name: impl Into<String>) -> HeadProjectsBuilder {
        HeadProjectsBuilder::default()
            .project_name(project_name.into())
            .to_owned()
    }
}
