use crate::model::AvailableExtension;
use crate::Result;
use async_trait::async_trait;
use semver::{Version, VersionReq};
use std::fmt::Debug;
use std::path::PathBuf;

/// A trait for archive repository implementations.
#[async_trait]
pub trait Repository: Debug + Send + Sync {
    /// Gets the name of the repository.
    fn name(&self) -> &str;

    /// Gets the available extensions.
    ///
    /// # Errors
    /// * if an error occurs while getting the extensions.
    async fn get_available_extensions(&self) -> Result<Vec<AvailableExtension>>;

    /// Gets the archive for the extension with the specified `name` and `version`.
    ///
    /// # Errors
    /// * if an error occurs while getting the archive.
    async fn get_archive(&self, name: &str, version: &VersionReq) -> Result<(Version, Vec<u8>)>;

    /// Installs the extension with the specified `name` and `version`.
    ///
    /// # Errors
    /// * if an error occurs while installing the extension.
    async fn install(
        &self,
        name: &str,
        library_dir: PathBuf,
        extension_dir: PathBuf,
        archive: &[u8],
    ) -> Result<Vec<PathBuf>>;
}

/// A struct representing an extension.
#[derive(Clone, Debug)]
pub struct ExtensionMetadata {
    namespace: String,
    name: String,
    description: String,
}

impl ExtensionMetadata {
    /// Creates a new extension.
    #[must_use]
    pub fn new(namespace: String, name: String, description: String) -> Self {
        Self {
            namespace,
            name,
            description,
        }
    }

    /// Gets the namespace of the extension.
    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Gets the name of the extension.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Gets the description of the extension.
    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }
}

/// A struct representing an extension.
#[derive(Clone, Debug)]
pub struct Extension {
    metadata: ExtensionMetadata,
    version: Version,
    bytes: Vec<u8>,
}

impl Extension {
    /// Creates a new extension.
    #[must_use]
    pub fn new(metadata: ExtensionMetadata, version: Version, bytes: Vec<u8>) -> Self {
        Self {
            metadata,
            version,
            bytes,
        }
    }

    /// Gets the extension metadata.
    #[must_use]
    pub fn metadata(&self) -> &ExtensionMetadata {
        &self.metadata
    }

    /// Gets the version of the extension.
    #[must_use]
    pub fn version(&self) -> &Version {
        &self.version
    }

    /// Gets the bytes of the extension.
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
    fn test_extension() {
        let namespace = "namespace".to_string();
        let name = "name".to_string();
        let description = "description".to_string();
        let metadata = ExtensionMetadata::new(namespace.clone(), name.clone(), description.clone());
        let version = Version::parse("1.0.0").unwrap();
        let bytes = vec![0, 1, 2, 3];
        let extension = Extension::new(metadata, version.clone(), bytes.clone());

        assert_eq!(namespace, extension.metadata.namespace());
        assert_eq!(name, extension.metadata.name());
        assert_eq!(description, extension.metadata.description());
        assert_eq!(&version, extension.version());
        assert_eq!(bytes.as_slice(), extension.bytes());
    }
}
