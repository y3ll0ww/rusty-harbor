use chrono::{DateTime, Utc};
use serde::Deserialize;

pub mod maps;
pub mod types;

use maps::{AdditionLinks, Annotations, ExtraAttrs, ScanOverview};
use types::{
    CveAllowlist, Label, ProjectMetadata, ProjectSummaryQuota, Reference, Registry, SbomOverview,
    Tag,
};

/// Response to requests that request project(s):
/// - [`GetProject`](crate::request::v2::project::get::GetProject)
/// - [`GetProjects`](crate::request::v2::project::get::GetProjects) (in a vector)
#[derive(Debug, Deserialize)]
pub struct Project {
    /// Project ID.
    pub project_id: u32,
    /// The owner ID of the project always means the creator of the project.
    pub owner_id: u32,
    /// The name of the project.
    pub name: String,
    /// The ID of referenced registry when the project is a proxy cache project.
    pub registry_id: Option<u64>,
    /// The creation time of the project.
    pub creation_time: DateTime<Utc>,
    /// The update time of the project.
    pub update_time: DateTime<Utc>,
    /// A deletion mark of the project.
    pub deleted: Option<bool>,
    /// The owner name of the project.
    pub owner_name: String,
    /// Correspond to the UI about whether the project's publicity is updatable (for UI).
    pub togglable: Option<bool>,
    /// The role ID with highest permission of the current user who triggered the API (for UI).
    /// This attribute is deprecated and will be removed in future versions.
    pub current_user_role_id: Option<u32>,
    /// The list of role ID of the current user who triggered the API (for UI).
    pub current_user_role_ids: Option<Vec<u32>>,
    /// The number of the repositories under this project.
    pub repo_count: u32,
    /// Project metadata.
    pub metadata: ProjectMetadata,
    /// The CVE Allowlist for system or project.
    pub cve_allowlist: CveAllowlist,
}

/// Response to the [`GetProjectDeletable`](crate::request::v2::project::get::GetProjectDeletable)
/// request.
#[derive(Debug, Deserialize)]
pub struct ProjectDeletable {
    /// Whether the project can be deleted.
    pub deletable: bool,
    /// The detail message when the project can not be deleted.
    pub message: String,
}

/// Respone to the [`GetProjectSummary`](crate::request::v2::project::get::GetProjectSummary) request.
#[derive(Debug, Deserialize)]
pub struct ProjectSummary {
    /// The number of the repositories under this project.
    pub repo_count: Option<usize>,
    /// The total number of project admin members.
    pub project_admin_count: Option<usize>,
    /// The total number of maintainer members.
    pub maintainer_count: Option<usize>,
    /// The total number of developer members.
    pub developer_count: Option<usize>,
    /// The total number of guest members.
    pub guest_count: Option<usize>,
    /// The total number of limited guest members.
    pub limited_guest_count: Option<usize>,
    pub quota: Option<ProjectSummaryQuota>,
    pub registry: Option<Registry>,
}

/// Response to the [`GetProjectArtifacts`](crate::request::v2::project::get::GetProjectArtifacts)
/// request (in a vector).
#[derive(Debug, Deserialize)]
pub struct Artifact {
    /// The ID of the artifact.
    pub id: i64,
    /// The type of the artifact, e.g. image, chart, etc.
    #[serde(rename = "type")]
    pub kind: Option<String>,
    /// The media type of the artifact.
    pub media_type: Option<String>,
    /// The manifest media type of the artifact.
    pub manifest_media_type: Option<String>,
    /// The artifact_type in the manifest of the artifact.
    pub artifact_type: Option<String>,
    /// The ID of the project that the artifact belongs to.
    pub project_id: Option<i64>,
    /// The ID of the repository that the artifact belongs to.
    pub repository_id: Option<i64>,
    /// The name of the repository that the artifact belongs to.
    pub repository_name: Option<String>,
    /// The digest of the artifact.
    pub digest: Option<String>,
    /// The size of the artifact.
    pub size: Option<i64>,
    /// The digest of the icon.
    pub icon: Option<String>,
    /// The push time of the artifact.
    pub push_time: Option<DateTime<Utc>>,
    /// The latest pull time of the artifact.
    pub pull_time: Option<DateTime<Utc>>,
    pub extra_attrs: Option<ExtraAttrs>,
    pub annotations: Option<Annotations>,
    pub references: Option<Vec<Reference>>,
    pub tags: Option<Vec<Tag>>,
    pub addition_links: Option<AdditionLinks>,
    pub labels: Option<Vec<Label>>,
    /// The scan overview attached in the metadata of tag.
    pub scan_overview: Option<ScanOverview>,
    /// The generate SBOM overview information.
    pub sbom_overview: Option<SbomOverview>,
    /// The accessory of the artifact.
    pub accessories: Option<Vec<Artifact>>,
}
