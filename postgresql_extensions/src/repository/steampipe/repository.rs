use crate::model::AvailableExtension;
use crate::repository::steampipe::URL;
use crate::repository::{steampipe, Repository};
use crate::Result;
use async_trait::async_trait;
use flate2::bufread::GzDecoder;
use postgresql_archive::{get_archive, matcher};
use semver::{Version, VersionReq};
use std::fmt::Debug;
use std::fs;
use std::io::Read;
use std::path::PathBuf;
use tar::Archive;

/// Steampipe repository.
#[derive(Debug)]
pub struct Steampipe;

impl Steampipe {
    /// Creates a new Steampipe repository.
    ///
    /// # Errors
    /// * If the repository cannot be created
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> Result<Box<dyn Repository>> {
        Ok(Box::new(Self))
    }

    /// Initializes the repository.
    pub fn initialize() {
        let _ = matcher::registry::register(|url| Ok(url == URL), steampipe::matcher);
    }
}

#[async_trait]
impl Repository for Steampipe {
    fn name(&self) -> &str {
        "steampipe"
    }

    async fn get_available_extensions(&self) -> Result<Vec<AvailableExtension>> {
        let mut extensions = Vec::new();
        for steampipe_extension in steampipe::extensions::get() {
            let extension = AvailableExtension::new(
                self.name(),
                steampipe_extension.name.as_str(),
                steampipe_extension.description.as_str(),
            );

            extensions.push(extension);
        }
        Ok(extensions)
    }

    async fn get_archive(&self, name: &str, version: &VersionReq) -> Result<(Version, Vec<u8>)> {
        let url = format!("{URL}/steampipe-plugin-{name}");
        let archive = get_archive(url.as_str(), version).await?;
        Ok(archive)
    }

    #[allow(clippy::case_sensitive_file_extension_comparisons)]
    async fn install(
        &self,
        _name: &str,
        library_dir: PathBuf,
        extension_dir: PathBuf,
        archive: &[u8],
    ) -> Result<Vec<PathBuf>> {
        let tar = GzDecoder::new(archive);
        let mut archive = Archive::new(tar);
        let mut files = Vec::new();

        for file in archive.entries()? {
            let mut file = file?;
            let file_path = file.path()?.to_path_buf();
            let file_name = file_path.to_string_lossy();

            if file_name.ends_with(".dylib") || file_name.ends_with(".so") {
                let mut bytes = Vec::new();
                file.read_to_end(&mut bytes)?;
                let path = PathBuf::from(&library_dir).join(file_path);
                fs::write(&path, bytes)?;
                files.push(path);
            } else if file_name.ends_with(".control") || file_name.ends_with(".sql") {
                let mut bytes = Vec::new();
                file.read_to_end(&mut bytes)?;
                let path = PathBuf::from(&extension_dir).join(file_path);
                fs::write(&path, bytes)?;
                files.push(path);
            }
        }

        Ok(files)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository::Repository;

    #[test]
    fn test_name() {
        let repository = Steampipe;
        assert_eq!("steampipe", repository.name());
    }

    #[tokio::test]
    async fn test_get_extensions() -> Result<()> {
        let repository = Steampipe;
        let extensions = repository.get_available_extensions().await?;
        let extension = &extensions[0];

        assert_eq!("abuseipdb", extension.name());
        assert_eq!(
            "Steampipe plugin to query IP address abuse data and more from AbuseIPDB.",
            extension.description()
        );
        assert_eq!(143, extensions.len());
        Ok(())
    }
}
