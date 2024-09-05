//! Manage PostgreSQL archives
#![allow(dead_code)]

use crate::error::Result;
use crate::{extractor, repository};
use regex::Regex;
use semver::{Version, VersionReq};
use std::path::{Path, PathBuf};
use tracing::instrument;

/// Gets the version for the specified [version requirement](VersionReq). If a version for the
/// [version requirement](VersionReq) is not found, then an error is returned.
///
/// # Errors
/// * If the version is not found.
#[instrument(level = "debug")]
pub async fn get_version(url: &str, version_req: &VersionReq) -> Result<Version> {
    let repository = repository::registry::get(url)?;
    let version = repository.get_version(version_req).await?;
    Ok(version)
}

/// Gets the archive for a given [version requirement](VersionReq) that passes the default
/// matcher. If no archive is found for the [version requirement](VersionReq) and matcher then
/// an [error](crate::error::Error) is returned.
///
/// # Errors
/// * If the archive is not found.
/// * If the archive cannot be downloaded.
#[instrument]
pub async fn get_archive(url: &str, version_req: &VersionReq) -> Result<(Version, Vec<u8>)> {
    let repository = repository::registry::get(url)?;
    let archive = repository.get_archive(version_req).await?;
    let version = archive.version().clone();
    let bytes = archive.bytes().to_vec();
    Ok((version, bytes))
}

/// Extracts the compressed tar `bytes` to the [out_dir](Path).
///
/// # Errors
/// Returns an error if the extraction fails.
#[instrument(skip(bytes))]
pub async fn extract(url: &str, bytes: &Vec<u8>, out_dir: &Path) -> Result<Vec<PathBuf>> {
    let extractor_fn = extractor::registry::get(url)?;
    let mut extract_directories = extractor::ExtractDirectories::default();
    extract_directories.add_mapping(Regex::new(".*")?, out_dir.to_path_buf());
    extractor_fn(bytes, extract_directories)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::configuration::theseus::URL;

    #[tokio::test]
    async fn test_get_version() -> Result<()> {
        let version_req = VersionReq::parse("=16.4.0")?;
        let version = get_version(URL, &version_req).await?;
        assert_eq!(Version::new(16, 4, 0), version);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_archive() -> Result<()> {
        let version_req = VersionReq::parse("=16.4.0")?;
        let (version, bytes) = get_archive(URL, &version_req).await?;
        assert_eq!(Version::new(16, 4, 0), version);
        assert!(!bytes.is_empty());
        Ok(())
    }
}
