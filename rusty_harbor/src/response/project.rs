use serde::Deserialize;

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
    pub creation_time: String,
    /// The update time of the project.
    pub update_time: String,
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
    pub creation_time: String,
    /// The update time of the allowlist.
    pub update_time: String,
}

/// The item in CVE allowlist.
#[derive(Debug, Deserialize)]
pub struct CveAllowlistItem {
    /// The ID of the CVE, such as "CVE-2019-10164"
    pub cve_id: String,
}
