use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_json::Value;

/// Response to requests that request project(s):
/// - [`GetProject`](crate::request::project::get::GetProject)
/// - [`GetProjects`](crate::request::project::get::GetProjects) (in a vector)
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

/// Response to the [`GetProjectDeletable`](crate::request::project::get::GetProjectDeletable)
/// request.
#[derive(Debug, Deserialize)]
pub struct ProjectDeletable {
    /// Whether the project can be deleted.
    pub deletable: bool,
    /// The detail message when the project can not be deleted.
    pub message: String,
}

/// Respone to the [`GetProjectSummary`](crate::request::project::get::GetProjectSummary) request.
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

/// Response to the [`GetProjectArtifacts`](crate::request::project::get::GetProjectArtifacts)
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

/// Project metadata.
#[derive(Debug, Deserialize)]
pub struct ProjectMetadata {
    /// The public status of the project. The valid values are "true", "false".
    pub public: String,
    /// Whether content trust is enabled or not. If it is enabled, user can't pull unsigned images
    /// from this project. The valid values are "true", "false".
    pub enable_content_trust: Option<String>,
    /// Whether cosign content trust is enabled or not. If it is enabled, user can't pull images
    /// without cosign signature from this project. The valid values are "true", "false".
    pub enable_content_trust_cosign: Option<String>,
    /// Whether prevent the vulnerable images from running. The valid values are "true", "false".
    pub prevent_vul: Option<String>,
    /// If the vulnerability is high than severity defined here, the images can't be pulled. The
    /// valid values are "none", "low", "medium", "high", "critical".
    pub severity: Option<String>,
    /// Whether scan images automatically when pushing. The valid values are "true", "false".
    pub auto_scan: Option<String>,
    /// Whether generating SBOM automatically when pushing a subject artifact. The valid values are
    /// "true", "false".
    pub auto_sbom_generation: Option<String>,
    /// Whether this project reuse the system level CVE allowlist as the allowlist of its own. The
    /// valid values are "true", "false". If it is set to "true" the actual allowlist associate
    /// with this project, if any, will be ignored.
    pub reuse_sys_cve_allowlist: Option<String>,
    /// The ID of the tag retention policy for the project.
    pub retention_id: Option<String>,
    /// The bandwidth limit of proxy cache, in Kbps (kilobits per second). It limits the
    /// communication between Harbor and the upstream registry, not the client and the Harbor.
    pub proxy_speed_kb: Option<String>,
}

/// The CVE Allowlist for system or project.
#[derive(Debug, Deserialize)]
pub struct CveAllowlist {
    /// ID of the allowlist.
    pub id: u32,
    /// ID of the project which the allowlist belongs to. For system level allowlist this attribute
    /// is zero.
    pub project_id: u32,
    /// The time for expiration of the allowlist, in the form of seconds since epoch. This is an
    /// optional attribute, if it's not set the CVE allowlist does not expire.
    pub expires_at: Option<u32>,
    /// The items in CVE allowlist.
    pub items: Vec<CveAllowlistItem>,
    /// The creation time of the allowlist.
    pub creation_time: DateTime<Utc>,
    /// The update time of the allowlist.
    pub update_time: DateTime<Utc>,
}

/// The item in CVE allowlist.
#[derive(Debug, Deserialize)]
pub struct CveAllowlistItem {
    /// The ID of the CVE, such as "CVE-2019-10164"
    pub cve_id: String,
}

#[derive(Debug, Deserialize)]
pub struct ProjectSummaryQuota{
    pub hard: Option<ResourceList>,
    pub used: Option<ResourceList>,
}

#[derive(Debug, Deserialize)]
pub struct Registry {
    /// The registry ID.
    pub id: i64,
    /// The registry URL string.
    pub url: String,
    /// The registry name.
    pub name: String,
    pub credential: RegistryCredential,
    /// Type of the registry, e.g. 'harbor'.
    #[serde(rename = "type")]
    pub kind: String,
    /// Whether or not the certificate will be verified when Harbor tries to access the server.
    pub insecure: bool,
    /// Description of the registry.
    pub description: String,
    /// Health status of the registry.
    pub status: String,
    /// The create time of the policy.
    pub creation_time: DateTime<Utc>,
    /// The update time of the policy.
    pub update_time: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct RegistryCredential {
    /// Credential type, such as 'basic', 'oauth'.
    #[serde(rename = "type")]
    pub kind: String,
    /// Access key, e.g. user name when credential type is 'basic'.
    pub access_key: String,
    /// Access secret, e.g. password when credential type is 'basic'.
    pub access_secret: String,
}

pub type ResourceList = HashMap<String, i64>;
pub type ExtraAttrs = HashMap<String, Value>;
pub type Annotations = HashMap<String, String>;
pub type AdditionLinks = HashMap<String, AdditionLink>;
/// The scan overview attached in the metadata of tag
pub type ScanOverview = HashMap<String, NativeReportSummary>;
pub type Summary = HashMap<String, usize>;

#[derive(Debug, Deserialize)]
pub struct AdditionLink {
    /// The link of the addition.
    pub href: String,
    /// Determine whether the link is an absolute URL or not.
    pub absolute: bool,
}

#[derive(Debug, Deserialize)]
pub struct Reference {
    /// The parent ID of the reference.
    pub parent_id: Option<i64>,
    /// The child ID of the reference.
    pub child_id: Option<i64>,
    /// The digest of the child artifact.
    pub child_digest: Option<String>,
    pub platform: Option<Platform>,
    pub annotations: Option<Annotations>,
    /// The download URLs.
    pub urls: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct Platform {
    /// The architecture that the artifact applys to.
    pub architecture: String,
    /// The OS that the artifact applys to.
    pub os: String,
    /// The version of the OS that the artifact applys to.
    #[serde(rename = "os.version")]
    pub os_version: Option<String>,
    /// The features of the OS that the artifact applys to.
    #[serde(rename = "os.features")]
    pub os_features: Option<Vec<String>>,
    /// The variant of the CPU.
    pub variant: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Tag {
    /// The ID of the tag
    pub id: i64,
    /// The ID of the repository that the tag belongs to.
    pub repository_id: i64,
    /// The ID of the artifact that the tag attached to.
    pub artifact_id: i64,
    /// The name of the tag.
    pub name: String,
    /// The push time of the tag.
    pub push_time: DateTime<Utc>,
    /// The latest pull time of the tag.
    pub pull_time: DateTime<Utc>,
    /// The immutable status of the tag.
    pub immutable: bool,
}

#[derive(Debug, Deserialize)]
pub struct Label {
    /// The ID of the label.
    pub id: i64,
    /// The name the label.
    pub name: Option<String>,
    /// The description the label.
    pub description: Option<String>,
    /// The color the label.
    pub color: Option<String>,
    /// The scope the label.
    pub scope: Option<String>,
    /// The ID of project that the label belongs to.
    pub project_id: Option<i64>,
    /// The creation time the label.
    pub creation_time: Option<DateTime<Utc>>,
    /// The update time of the label.
    pub update_time: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct NativeReportSummary {
    /// ID of the native scan report.
    /// Example: 5f62c830-f996-11e9-957f-0242c0a89008
    pub report_id: Option<String>,
    /// The status of the report generating process
    /// Example: Success
    pub scan_status: Option<String>,
    /// The overall severity.
    /// Example: High
    pub severity: Option<String>,
    /// The seconds spent for generating the report.
    /// Example: 300
    pub duration: Option<i64>,
    /// Contains the total number of the foun d vulnerabilities number and numbers of each severity
    /// level.
    pub summary: Option<VulnerabilitySummary>,
    /// The start time of the scan process that generating report.
    /// Example: 2006-01-02T14:04:05Z
    pub start_time: Option<DateTime<Utc>>,
    /// The end time of the scan process that generating report.
    /// Example: 2006-01-02T15:04:05Z
    pub end_time: Option<DateTime<Utc>>,
    /// The complete percent of the scanning which value is between 0 and 100
    /// Example: 100
    pub complete_percent: Option<usize>,
    pub scanner: Option<Scanner>,
}

/// Contains the total number of the foun d vulnerabilities number and numbers of each severity
/// level.
#[derive(Debug, Deserialize)]
pub struct VulnerabilitySummary {
    /// The total number of the found vulnerabilities.
    /// Example: 500
    pub total: Option<usize>,
    /// The number of the fixable vulnerabilities.
    /// Example: 100
    pub fixable: Option<usize>,
    /// Numbers of the vulnerabilities with different severity.
    /// Example: { "Critical": 5, "High": 5 }
    pub summary: Option<Summary>,
}

#[derive(Debug, Deserialize)]
pub struct Scanner {
    /// Name of the scanner.
    /// Example: Trivy
    pub name: Option<String>,
    /// Name of the scanner provider.
    /// Example: Aqua Security
    pub vendor: Option<String>,
    /// Version of the scanner adapter.
    /// Example: v0.9.1
    pub version: Option<String>,
}

/// The generate SBOM overview information.
#[derive(Debug, Deserialize)]
pub struct SbomOverview {
    /// The start time of the generating sbom report task.
    /// Example: 2006-01-02T14:04:05Z
    pub start_time: Option<DateTime<Utc>>,
    /// The end time of the generating sbom report task.
    /// Example: 2006-01-02T15:04:05Z
    pub end_time: Option<DateTime<Utc>>,
    /// The status of the generating SBOM task.
    pub scan_status: Option<String>,
    /// The digest of the generated SBOM accessory.
    pub sbom_digest: Option<String>,
    /// ID of the native scan report.
    /// Example: 5f62c830-f996-11e9-957f-0242c0a89008
    pub report_id: Option<String>,
    /// Time in seconds required to create the report.
    /// Example: 300
    pub duration: Option<i64>,
    pub scanner: Option<Scanner>,
}

/// The accessory of the artifact.
#[derive(Debug, Deserialize)]
pub struct Accessory {
    /// The ID of the accessory.
    pub id: i64,
    /// The artifact id of the accessory.
    pub artifact_id: i64,
    /// Going to be deprecated, use repo and digest for insteand. The subject artifact id of the
    /// accessory.
    pub subject_artifact_id: Option<i64>,
    /// The subject artifact digest of the accessory.
    pub subject_artifact_digest: Option<String>,
    /// The subject artifact repository name of the accessory.
    pub subject_artifact_repo: Option<String>,
    /// The artifact size of the accessory.
    pub size: Option<i64>,
    /// The artifact digest of the accessory.
    pub digest: Option<String>,
    /// The artifact size of the accessory.
    #[serde(rename = "type")]
    pub kind: Option<String>,
    /// The icon of the accessory.
    pub icon: Option<String>,
    /// The creation time of the accessory.
    pub creation_time: Option<DateTime<Utc>>,
}
