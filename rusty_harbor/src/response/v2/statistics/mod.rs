use serde::Deserialize;

/// Response to the [`GetStatistics`](crate::request::v2::statistics::get::GetStatistics) request.
#[derive(Debug, Deserialize)]
pub struct Statistic {
    /// The count of the private projects.
    pub private_project_count: u64,
    /// The count of the private repositories.
    pub private_repo_count: u64,
    /// The count of the public projects.
    pub public_project_count: u64,
    /// The count of the public repositories.
    pub public_repo_count: u64,
    /// The count of the total projects, only be seen by the system admin.
    pub total_project_count: u64,
    /// The count of the total repositories, only be seen by the system admin.
    pub total_repo_count: u64,
    /// The total storage consumption of blobs, only be seen by the system admin.
    pub total_storage_consumption: u64,
}
