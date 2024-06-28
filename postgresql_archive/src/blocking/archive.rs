use crate::{Version, VersionReq};
use bytes::Bytes;
use std::path::Path;
use tokio::runtime::Runtime;

lazy_static! {
    static ref RUNTIME: Runtime = Runtime::new().unwrap();
}

/// Gets the version for the specified [version requirement](VersionReq). If a version for the
/// [version requirement](VersionReq) is not found, then an error is returned.
///
/// # Arguments
/// * `url` - The URL to released archives.
/// * `version_req` - The version requirement.
///
/// # Returns
/// * The version matching the requirement.
///
/// # Errors
/// * If the version is not found.
pub fn get_version(url: &str, version_req: &VersionReq) -> crate::Result<Version> {
    RUNTIME
        .handle()
        .block_on(async move { crate::get_version(url, version_req).await })
}

/// Gets the archive for a given [version requirement](VersionReq) that passes the default
/// matcher. If no archive is found for the [version requirement](VersionReq) and matcher then
/// an [error](crate::error::Error) is returned.
///
/// # Arguments
/// * `url` - The URL to the archive resources.
/// * `version_req` - The version requirement.
///
/// # Returns
/// * The archive version and bytes.
///
/// # Errors
/// * If the archive is not found.
/// * If the archive cannot be downloaded.
pub fn get_archive(url: &str, version_req: &VersionReq) -> crate::Result<(Version, Bytes)> {
    RUNTIME
        .handle()
        .block_on(async move { crate::get_archive(url, version_req).await })
}

/// Extracts the compressed tar [bytes](Bytes) to the [out_dir](Path).
///
/// # Arguments
/// * `bytes` - The compressed tar bytes.
/// * `out_dir` - The directory to extract the tar to.
///
/// # Returns
/// * The extracted files.
///
/// # Errors
/// Returns an error if the extraction fails.
pub fn extract(bytes: &Bytes, out_dir: &Path) -> crate::Result<()> {
    RUNTIME
        .handle()
        .block_on(async move { crate::extract(bytes, out_dir).await })
}
