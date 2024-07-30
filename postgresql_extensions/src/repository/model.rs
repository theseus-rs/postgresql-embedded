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
