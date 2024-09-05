use crate::{Version, VersionReq};
use std::path::{Path, PathBuf};
use std::sync::LazyLock;
use tokio::runtime::Runtime;

static RUNTIME: LazyLock<Runtime> = LazyLock::new(|| Runtime::new().unwrap());

/// Gets the version for the specified [version requirement](VersionReq). If a version for the
/// [version requirement](VersionReq) is not found, then an error is returned.
///
/// # Errors
/// * If the version is not found.
pub fn get_version(url: &str, version_req: &VersionReq) -> crate::Result<Version> {
    RUNTIME
        .handle()
        .block_on(async move { crate::get_version(url, version_req).await })
}

/// Gets the archive for a given [version requirement](VersionReq) that passes the default
/// matcher.
///
/// If no archive is found for the [version requirement](VersionReq) and matcher then
/// an [error](crate::error::Error) is returned.
///
/// # Errors
/// * If the archive is not found.
/// * If the archive cannot be downloaded.
pub fn get_archive(url: &str, version_req: &VersionReq) -> crate::Result<(Version, Vec<u8>)> {
    RUNTIME
        .handle()
        .block_on(async move { crate::get_archive(url, version_req).await })
}

/// Extracts the compressed tar `bytes` to the [out_dir](Path).
///
/// # Errors
/// Returns an error if the extraction fails.
pub fn extract(url: &str, bytes: &Vec<u8>, out_dir: &Path) -> crate::Result<Vec<PathBuf>> {
    RUNTIME
        .handle()
        .block_on(async move { crate::extract(url, bytes, out_dir).await })
}
