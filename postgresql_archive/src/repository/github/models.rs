//! Structs for GitHub API responses
use serde::{Deserialize, Serialize};

/// Represents a GitHub release
#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct Release {
    pub url: String,
    pub assets_url: String,
    pub upload_url: String,
    pub html_url: String,
    pub id: i64,
    pub tag_name: String,
    pub name: String,
    pub draft: bool,
    pub prerelease: bool,
    pub assets: Vec<Asset>,
}

/// Represents a GitHub asset
#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct Asset {
    pub url: String,
    pub id: i64,
    pub node_id: String,
    pub name: String,
    pub label: String,
    pub content_type: String,
    pub state: String,
    pub size: i64,
    pub browser_download_url: String,
}
