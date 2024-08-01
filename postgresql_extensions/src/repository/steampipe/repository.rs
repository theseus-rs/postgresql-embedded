use crate::model::AvailableExtension;
use crate::repository::steampipe::URL;
use crate::repository::{steampipe, Repository};
use crate::Error::ExtensionNotFound;
use crate::Result;
use async_trait::async_trait;
use flate2::bufread::GzDecoder;
use postgresql_archive::repository::github::repository::GitHub;
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
    ///
    /// # Errors
    /// * If the repository cannot be initialized.
    pub fn initialize() -> Result<()> {
        matcher::registry::register(|url| Ok(url.starts_with(URL)), steampipe::matcher)?;
        postgresql_archive::repository::registry::register(
            |url| Ok(url.starts_with(URL)),
            Box::new(GitHub::new),
        )?;
        Ok(())
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

    async fn get_archive(
        &self,
        postgresql_version: &str,
        name: &str,
        version: &VersionReq,
    ) -> Result<(Version, Vec<u8>)> {
        let Some(extension) = steampipe::extensions::get()
            .iter()
            .find(|extension| extension.name == name)
        else {
            let extension = format!("{}:{}:{}", self.name(), name, version);
            return Err(ExtensionNotFound(extension));
        };
        let url = format!("{}?postgresql_version={postgresql_version}", extension.url);
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
            let file_path = PathBuf::from(file.path()?.file_name().unwrap_or_default());
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
    async fn test_get_available_extensions() -> Result<()> {
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

    #[tokio::test]
    async fn test_get_archive_error() -> anyhow::Result<()> {
        let repository = Steampipe;
        let postgresql_version = "15.7";
        let name = "does-not-exist";
        let version = VersionReq::parse("=0.12.0")?;
        let result = repository
            .get_archive(postgresql_version, name, &version)
            .await;
        assert!(result.is_err());
        Ok(())
    }
}
