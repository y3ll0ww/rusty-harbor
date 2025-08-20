use serde::Deserialize;

use crate::response::v2::project::Project;

/// An array of search results.
/// Response to the [`GetSearch`](crate::request::v2::search::get::GetSearch) request.
#[derive(Debug, Deserialize)]
pub struct Search {
    /// Search results of the projects that matched the filter keywords.
    pub project: Option<Vec<Project>>,
    /// Search results of the repositories that matched the filter keywords.
    pub repository: Option<Vec<SearchRepository>>,
}

/// The health status of component
#[derive(Debug, Deserialize)]
pub struct SearchRepository {
    /// The ID of the project that the repository belongs to.
    pub project_id: u32,
    /// The name of the project that the repository belongs to.
    pub project_name: String,
    /// The flag to indicate the publicity of the project that the repository belongs to (1 is
    /// public, 0 is not).
    pub project_public: bool,
    /// The name of the repository.
    pub repository_name: String,
    /// The count how many times the repository is pulled.
    pub pull_count: u32,
    /// The count of artifacts in the repository.
    pub artifact_count: u32,
}
