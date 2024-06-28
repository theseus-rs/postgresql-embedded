use async_trait::async_trait;
use semver::{Version, VersionReq};
use std::fmt::Debug;

/// A trait for archive repository implementations.
#[async_trait]
pub trait Repository: Debug + Send + Sync {
    /// Gets the name of the repository.
    ///
    /// # Returns
    /// * The name of the repository.
    fn name(&self) -> &str;

    /// Gets the version for the specified [version requirement](VersionReq). If a
    /// [version](Version) for the [version requirement](VersionReq) is not found,
    /// then an error is returned.
    ///
    /// # Arguments
    /// * `version_req` - The version requirement.
    ///
    /// # Returns
    /// * The version matching the requirement.
    ///
    /// # Errors
    /// * If the version is not found.
    async fn get_version(&self, version_req: &VersionReq) -> crate::Result<Version>;

    /// Gets the archive for a given [version requirement](VersionReq) that passes the default
    /// matcher. If no archive is found for the [version requirement](VersionReq) and matcher then
    /// an [error](crate::error::Error) is returned.
    ///
    /// # Arguments
    /// * `version_req` - The version requirement.
    ///
    /// # Returns
    /// * The archive version and bytes.
    ///
    /// # Errors
    /// * If the archive is not found.
    /// * If the archive cannot be downloaded.
    async fn get_archive(&self, version_req: &VersionReq) -> crate::Result<Archive>;
}

/// A struct representing an archive.
#[derive(Clone, Debug)]
pub struct Archive {
    name: String,
    version: Version,
    bytes: Vec<u8>,
}

impl Archive {
    /// Creates a new archive.
    ///
    /// # Arguments
    /// * `name` - The name of the archive.
    /// * `version` - The version of the archive.
    /// * `bytes` - The bytes of the archive.
    ///
    /// # Returns
    /// * The archive.
    #[must_use]
    pub fn new(name: String, version: Version, bytes: Vec<u8>) -> Self {
        Self {
            name,
            version,
            bytes,
        }
    }

    /// Gets the name of the archive.
    ///
    /// # Returns
    /// * The name of the archive.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Gets the version of the archive.
    ///
    /// # Returns
    /// * The version of the archive.
    #[must_use]
    pub fn version(&self) -> &Version {
        &self.version
    }

    /// Gets the bytes of the archive.
    ///
    /// # Returns
    /// * The bytes of the archive.
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use semver::Version;

    #[test]
    fn test_archive() {
        let name = "test".to_string();
        let version = Version::parse("1.0.0").unwrap();
        let bytes = vec![0, 1, 2, 3];
        let archive = Archive::new(name.clone(), version.clone(), bytes.clone());
        assert_eq!(archive.name(), name);
        assert_eq!(archive.version(), &version);
        assert_eq!(archive.bytes(), bytes.as_slice());
    }
}
