use crate::Version;
use bytes::Bytes;
use std::path::Path;
use tokio::runtime::Runtime;

lazy_static! {
    static ref RUNTIME: Runtime = Runtime::new().unwrap();
}

/// Gets the version of PostgreSQL for the specified [`version`](Version).  If the version minor or release is not
/// specified, then the latest version is returned. If a release for the [`version`](Version) is not found, then a
/// [`ReleaseNotFound`] error is returned.
pub fn get_version(version: &Version) -> crate::Result<Version> {
    RUNTIME
        .handle()
        .block_on(async move { crate::get_version(version).await })
}

/// Gets the archive for a given [`version`](Version) of PostgreSQL for the current target.
/// If the [`version`](Version) is not found for this target, then an [error](Archive) is returned.
///
/// Returns the archive bytes and the archive hash.
pub fn get_archive(version: &Version) -> crate::Result<(Version, Bytes, String)> {
    RUNTIME
        .handle()
        .block_on(async move { crate::get_archive(version).await })
}

/// Gets the archive for a given [`version`](Version) of PostgreSQL and `target` (e.g. `x86_64-unknown-linux-gnu`).
/// If the [`version`](Version) or `target` is not found, then an [error](ArchiveError) is returned.
///
/// Returns the archive bytes and the archive hash.
pub fn get_archive_for_target<S: AsRef<str>>(
    version: &Version,
    target: S,
) -> crate::Result<(Version, Bytes, String)> {
    RUNTIME
        .handle()
        .block_on(async move { crate::get_archive_for_target(version, target).await })
}

/// Extracts the compressed tar `bytes` to the `out_dir`.
pub fn extract(bytes: &Bytes, out_dir: &Path) -> crate::Result<()> {
    RUNTIME
        .handle()
        .block_on(async move { crate::extract(bytes, out_dir).await })
}
