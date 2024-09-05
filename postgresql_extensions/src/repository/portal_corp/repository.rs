use crate::matcher::zip_matcher;
use crate::model::AvailableExtension;
use crate::repository::portal_corp::URL;
use crate::repository::Repository;
use crate::Result;
use async_trait::async_trait;
use postgresql_archive::extractor::{zip_extract, ExtractDirectories};
use postgresql_archive::get_archive;
use postgresql_archive::repository::github::repository::GitHub;
use regex::Regex;
use semver::{Version, VersionReq};
use std::fmt::Debug;
use std::path::PathBuf;

/// PortalCorp repository.
#[derive(Debug)]
pub struct PortalCorp;

impl PortalCorp {
    /// Creates a new PortalCorp repository.
    ///
    /// # Errors
    /// * If the repository cannot be created
    #[expect(clippy::new_ret_no_self)]
    pub fn new() -> Result<Box<dyn Repository>> {
        Ok(Box::new(Self))
    }

    /// Initializes the repository.
    ///
    /// # Errors
    /// * If the repository cannot be initialized.
    pub fn initialize() -> Result<()> {
        postgresql_archive::matcher::registry::register(
            |url| Ok(url.starts_with(URL)),
            zip_matcher,
        )?;
        postgresql_archive::repository::registry::register(
            |url| Ok(url.starts_with(URL)),
            Box::new(GitHub::new),
        )?;
        Ok(())
    }
}

#[async_trait]
impl Repository for PortalCorp {
    fn name(&self) -> &str {
        "portal-corp"
    }

    async fn get_available_extensions(&self) -> Result<Vec<AvailableExtension>> {
        let extensions = vec![AvailableExtension::new(
            self.name(),
            "pgvector_compiled",
            "Precompiled OS packages for pgvector",
        )];
        Ok(extensions)
    }

    async fn get_archive(
        &self,
        postgresql_version: &str,
        name: &str,
        version: &VersionReq,
    ) -> Result<(Version, Vec<u8>)> {
        let url = format!("{URL}/{name}?postgresql_version={postgresql_version}");
        let archive = get_archive(url.as_str(), version).await?;
        Ok(archive)
    }

    async fn install(
        &self,
        _name: &str,
        library_dir: PathBuf,
        extension_dir: PathBuf,
        archive: &[u8],
    ) -> Result<Vec<PathBuf>> {
        let mut extract_directories = ExtractDirectories::default();
        extract_directories.add_mapping(Regex::new(r"\.(dll|dylib|so)$")?, library_dir);
        extract_directories.add_mapping(Regex::new(r"\.(control|sql)$")?, extension_dir);
        let bytes = &archive.to_vec();
        let files = zip_extract(bytes, extract_directories)?;
        Ok(files)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository::Repository;

    #[test]
    fn test_name() {
        let repository = PortalCorp;
        assert_eq!("portal-corp", repository.name());
    }

    #[tokio::test]
    async fn test_get_available_extensions() -> Result<()> {
        let repository = PortalCorp;
        let extensions = repository.get_available_extensions().await?;
        let extension = &extensions[0];

        assert_eq!("pgvector_compiled", extension.name());
        assert_eq!(
            "Precompiled OS packages for pgvector",
            extension.description()
        );
        Ok(())
    }
}
