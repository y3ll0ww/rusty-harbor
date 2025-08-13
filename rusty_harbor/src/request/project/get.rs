use derive_builder::Builder;
use query_url::QueryUrl;
use serde::Serialize;

use crate::request::ToUrl;

/// This endpoint returns specific project information by project ID.
#[derive(QueryUrl, Serialize)]
#[query_url(path = "projects/{project_name_or_id}")]
pub struct GetProject {
    /// The name or id of the project.
    #[serde(skip)]
    pub project_name_or_id: String,
}

impl GetProject {
    pub fn new(project_name_or_id: impl Into<String>) -> Self {
        GetProject {
            project_name_or_id: project_name_or_id.into(),
        }
    }
}

/// Get the deletable status of the project.
#[derive(QueryUrl, Serialize)]
#[query_url(path = "projects/{project_name_or_id}/_deletable")]
pub struct GetProjectDeletable {
    /// The name or id of the project.
    #[serde(skip)]
    pub project_name_or_id: String,
}

impl GetProjectDeletable {
    pub fn new(project_name_or_id: impl Into<String>) -> Self {
        GetProjectDeletable {
            project_name_or_id: project_name_or_id.into(),
        }
    }
}

/// Get summary of the project.
#[derive(QueryUrl, Serialize)]
#[query_url(path = "projects/{project_name_or_id}/summary")]
pub struct GetProjectSummary {
    /// The name or id of the project.
    #[serde(skip)]
    pub project_name_or_id: String,
}

impl GetProjectSummary {
    pub fn new(project_name_or_id: impl Into<String>) -> Self {
        GetProjectSummary {
            project_name_or_id: project_name_or_id.into(),
        }
    }
}

/// List artifacts of the specified project.
#[derive(Builder, Default, QueryUrl, Serialize)]
#[builder(setter(into, strip_option), pattern = "owned")]
#[query_url(path = "projects/{project_name_or_id}/artifacts")]
pub struct GetProjectArtifacts {
    /// The name or id of the project.
    #[serde(skip)]
    pub project_name_or_id: String,
    /// Query string to query resources. Supported query patterns are "exact match(k=v)",
    /// "fuzzy match(k=~v)", "range(k=[min~max])", "list with union releationship(k={v1 v2 v3})"
    /// and "list with intersetion relationship(k=(v1 v2 v3))". The value of range and list can be
    /// string(enclosed by " or '), integer or time(in format "2020-04-09 02:36:00"). All of these
    /// query patterns should be put in the query string "q=xxx" and splitted by ",". e.g.
    /// q=k1=v1,k2=~v2,k3=[min~max].
    #[builder(default)]
    #[serde(rename = "q")]
    pub query: Option<String>,
    /// Sort the resource list in ascending or descending order. e.g. sort by field1 in ascending
    /// order and field2 in descending order with "sort=field1,-field2".
    #[builder(default)]
    pub sort: Option<String>,
    /// The page number.
    /// Default value: `1`
    #[builder(default)]
    pub page: Option<i32>,
    /// The size of per page.
    /// Default value: `10`
    #[builder(default)]
    pub page_size: Option<i32>,
    /// Specify whether the tags are included inside the returning artifacts.
    /// Default value: `true`
    #[builder(default)]
    pub with_tag: Option<bool>,
    /// Specify whether the labels are included inside the returning artifacts.
    /// Default value: `false`
    #[builder(default)]
    pub with_label: Option<bool>,
    /// Specify whether the scan overview is included inside the returning artifacts.
    /// Default value: `false`
    #[builder(default)]
    pub with_scan_overview: Option<bool>,
    /// Specify whether the SBOM overview is included in returning artifacts, when this option is
    /// true, the SBOM overview will be included in the response.
    /// Default value: `false`
    #[builder(default)]
    pub with_sbom_overview: Option<bool>,
    /// Specify whether the immutable status is included inside the tags of the returning artifacts.
    /// Only works when setting "with_immutable_status=true".
    /// Default value: `false`
    #[builder(default)]
    pub with_immutable_status: Option<bool>,
    /// Specify whether the accessories are included of the returning artifacts. Only works when
    /// setting "with_accessory=true".
    /// Default value: `false`
    #[builder(default)]
    pub with_accessory: Option<bool>,
    /// Specify whether only the latest pushed artifact of each repository is included inside the
    /// returning artifacts. Only works when either artifact_type or media_type is included in the
    /// query.
    /// Default value: `false`
    #[builder(default)]
    pub latest_in_repository: Option<bool>,
}

impl GetProjectArtifacts {
    pub fn builder(project_name_or_id: impl Into<String>) -> GetProjectArtifactsBuilder {
        GetProjectArtifactsBuilder::default().project_name_or_id(project_name_or_id)
    }
}

/// This endpoint returns projects created by Harbor.
#[derive(Builder, Default, QueryUrl, Serialize)]
#[builder(setter(into, strip_option), pattern = "owned")]
#[query_url(path = "projects")]
pub struct GetProjects {
    /// Query string to query resources. Supported query patterns are "exact match(k=v)",
    /// "fuzzy match(k=~v)", "range(k=[min~max])", "list with union releationship(k={v1 v2 v3})"
    /// and "list with intersetion relationship(k=(v1 v2 v3))". The value of range and list can be
    /// string(enclosed by " or '), integer or time(in format "2020-04-09 02:36:00"). All of these
    /// query patterns should be put in the query string "q=xxx" and splitted by ",". e.g.
    /// q=k1=v1,k2=~v2,k3=[min~max].
    #[builder(default)]
    #[serde(rename = "q")]
    pub query: Option<String>,
    /// The page number.
    /// Default value: `1`
    #[builder(default)]
    pub page: Option<i32>,
    /// The size of per page.
    /// Default value: `10`
    #[builder(default)]
    pub page_size: Option<i32>,
    /// Sort the resource list in ascending or descending order. e.g. sort by field1 in ascending
    /// order and field2 in descending order with "sort=field1,-field2".
    #[builder(default)]
    pub sort: Option<String>,
    /// The name of project.
    #[builder(default)]
    pub name: Option<String>,
    /// The project is public or private.
    #[builder(default)]
    pub public: Option<bool>,
    /// The name of project owner.
    #[builder(default)]
    pub owner: Option<String>,
    /// Bool value indicating whether return detailed information of the project.
    /// Default value: `true`
    #[builder(default)]
    pub with_detail: Option<bool>,
}

impl GetProjects {
    pub fn builder() -> GetProjectsBuilder {
        GetProjectsBuilder::default()
    }
}
