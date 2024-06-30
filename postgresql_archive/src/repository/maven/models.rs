/// Maven metadata XML structure
///
/// ```xml
/// <metadata>
///   <groupId>io.zonky.test.postgres</groupId>
///   <artifactId>embedded-postgres-binaries-linux-amd64</artifactId>
///   <versioning>
///     <latest>16.2.0</latest>
///     <release>16.2.0</release>
///     <versions>
///       ...
///       <version>15.6.0</version>
///       <version>16.2.0</version>
///     </versions>
///     <lastUpdated>20240210235512</lastUpdated>
///   </versioning>
/// </metadata>
/// ```
use serde::{Deserialize, Serialize};

/// Represents a Maven artifact metadata
#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct Metadata {
    #[serde(rename = "groupId")]
    pub(crate) group_id: String,
    #[serde(rename = "artifactId")]
    pub(crate) artifact_id: String,
    pub(crate) versioning: Versioning,
}

/// Represents Maven versioning information
#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct Versioning {
    pub(crate) latest: String,
    pub(crate) release: String,
    pub(crate) versions: Versions,
    #[serde(rename = "lastUpdated")]
    pub(crate) last_updated: String,
}

/// Represents Maven versions
#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct Versions {
    pub(crate) version: Vec<String>,
}
