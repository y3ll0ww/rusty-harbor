use derive_builder::Builder;
use derive_harbor::Harbor;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde::Serialize;

use crate::{
    request::HarborRequest,
    response::v2::project::{Artifact, Project, ProjectDeletable, ProjectSummary},
};

/// This endpoint returns specific project information by project ID.
#[derive(Builder, Harbor, Serialize)]
#[builder(setter(into, strip_option), pattern = "owned")]
#[harbor(
    url = "projects/{project_name_or_id}",
    response = Project,
)]
pub struct GetProject {
    /// An unique ID for the request.
    #[builder(default)]
    #[header(rename = "X-Request-Id")]
    pub request_id: Option<String>,
    /// The flag to indicate whether the parameter which supports both name and id in the path is
    /// the name of the resource. When the X-Is-Resource-Name is false and the parameter can be
    /// converted to an integer, the parameter will be as an id, otherwise, it will be as a name.
    /// Default value: false
    #[builder(default)]
    #[header(rename = "X-Is-Resource-Name")]
    pub is_resource_name: Option<bool>,
    /// The name or id of the project.
    #[serde(skip)]
    pub project_name_or_id: String,
}

impl GetProject {
    pub fn builder(project_name_or_id: impl Into<String>) -> GetProjectBuilder {
        GetProjectBuilder::default().project_name_or_id(project_name_or_id)
    }
}

/// Get the deletable status of the project.
#[derive(Builder, Harbor, Serialize)]
#[builder(setter(into, strip_option), pattern = "owned")]
#[harbor(
    url = "projects/{project_name_or_id}/_deletable",
    response = ProjectDeletable,
)]
pub struct GetProjectDeletable {
    /// An unique ID for the request.
    #[builder(default)]
    #[header(rename = "X-Request-Id")]
    pub request_id: Option<String>,
    /// The flag to indicate whether the parameter which supports both name and id in the path is
    /// the name of the resource. When the X-Is-Resource-Name is false and the parameter can be
    /// converted to an integer, the parameter will be as an id, otherwise, it will be as a name.
    /// Default value: false
    #[builder(default)]
    #[header(rename = "X-Is-Resource-Name")]
    pub is_resource_name: Option<bool>,
    /// The name or id of the project.
    #[serde(skip)]
    pub project_name_or_id: String,
}

impl GetProjectDeletable {
    pub fn builder(project_name_or_id: impl Into<String>) -> GetProjectDeletableBuilder {
        GetProjectDeletableBuilder::default().project_name_or_id(project_name_or_id)
    }
}

/// Get summary of the project.
#[derive(Builder, Harbor, Serialize)]
#[builder(setter(into, strip_option), pattern = "owned")]
#[harbor(
    url = "projects/{project_name_or_id}/summary",
    response = ProjectSummary,
)]
pub struct GetProjectSummary {
    /// An unique ID for the request.
    #[builder(default)]
    #[header(rename = "X-Request-Id")]
    pub request_id: Option<String>,
    /// The flag to indicate whether the parameter which supports both name and id in the path is
    /// the name of the resource. When the X-Is-Resource-Name is false and the parameter can be
    /// converted to an integer, the parameter will be as an id, otherwise, it will be as a name.
    /// Default value: false
    #[builder(default)]
    #[header(rename = "X-Is-Resource-Name")]
    pub is_resource_name: Option<bool>,
    /// The name or id of the project.
    #[serde(skip)]
    pub project_name_or_id: String,
}

impl GetProjectSummary {
    pub fn builder(project_name_or_id: impl Into<String>) -> GetProjectSummaryBuilder {
        GetProjectSummaryBuilder::default().project_name_or_id(project_name_or_id)
    }
}

/// List artifacts of the specified project.
#[derive(Builder, Default, Harbor, Serialize)]
#[builder(setter(into, strip_option), pattern = "owned")]
#[harbor(
    url = "projects/{project_name_or_id}/artifacts",
    response = Vec<Artifact>,
)]
pub struct GetProjectArtifacts {
    /// An unique ID for the request.
    #[builder(default)]
    #[header(rename = "X-Request-Id")]
    pub request_id: Option<String>,
    /// The flag to indicate whether the parameter which supports both name and id in the path is
    /// the name of the resource. When the X-Is-Resource-Name is false and the parameter can be
    /// converted to an integer, the parameter will be as an id, otherwise, it will be as a name.
    /// Default value: false
    #[builder(default)]
    #[header(rename = "X-Is-Resource-Name")]
    pub is_resource_name: Option<bool>,
    /// A comma-separated lists of MIME types for the scan report or scan summary. The first mime
    /// type will be used when the report found for it.
    ///
    /// Currently the mime type supports 'application/vnd.scanner.adapter.vuln.report.harbor+json;
    /// version=1.0' and 'application/vnd.security.vulnerability.report; version=1.1'
    ///
    /// Default value: application/vnd.security.vulnerability.report; version=1.1,
    /// application/vnd.scanner.adapter.vuln.report.harbor+json; version=1.0
    #[builder(default)]
    #[header(rename = "X-Accept-Vulnerabilities")]
    pub accept_vulnerabilities: Option<String>,
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
    pub q: Option<String>,
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
#[derive(Builder, Default, Harbor, Serialize)]
#[builder(setter(into, strip_option), pattern = "owned")]
#[harbor(
    url = "projects",
    response = Vec<Project>,
)]
pub struct GetProjects {
    /// An unique ID for the request.
    #[builder(default)]
    #[header(rename = "X-Request-Id")]
    pub request_id: Option<String>,
    /// Query string to query resources. Supported query patterns are "exact match(k=v)",
    /// "fuzzy match(k=~v)", "range(k=[min~max])", "list with union releationship(k={v1 v2 v3})"
    /// and "list with intersetion relationship(k=(v1 v2 v3))". The value of range and list can be
    /// string(enclosed by " or '), integer or time(in format "2020-04-09 02:36:00"). All of these
    /// query patterns should be put in the query string "q=xxx" and splitted by ",". e.g.
    /// q=k1=v1,k2=~v2,k3=[min~max].
    #[builder(default)]
    pub q: Option<String>,
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
