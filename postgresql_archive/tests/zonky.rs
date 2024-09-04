#[cfg(feature = "zonky")]
use postgresql_archive::configuration::zonky;
#[cfg(feature = "zonky")]
use postgresql_archive::extract;
#[cfg(feature = "zonky")]
use postgresql_archive::{get_archive, get_version};
#[cfg(feature = "zonky")]
use semver::VersionReq;
#[cfg(feature = "zonky")]
use std::fs::remove_dir_all;
#[cfg(feature = "zonky")]
use test_log::test;

#[test(tokio::test)]
#[cfg(feature = "zonky")]
async fn test_get_version_not_found() -> postgresql_archive::Result<()> {
    let invalid_version_req = VersionReq::parse("=1.0.0")?;
    let result = get_version(zonky::URL, &invalid_version_req).await;

    assert!(result.is_err());
    Ok(())
}

#[test(tokio::test)]
#[cfg(feature = "zonky")]
async fn test_get_version() -> anyhow::Result<()> {
    let version_req = VersionReq::parse("=16.2.0")?;
    let latest_version = get_version(zonky::URL, &version_req).await?;

    assert!(version_req.matches(&latest_version));
    Ok(())
}

#[test(tokio::test)]
#[cfg(feature = "zonky")]
async fn test_get_archive_and_extract() -> anyhow::Result<()> {
    let url = zonky::URL;
    let version_req = VersionReq::parse("=16.4.0")?;
    let (archive_version, archive) = get_archive(url, &version_req).await?;

    assert!(version_req.matches(&archive_version));

    let out_dir = tempfile::tempdir()?.path().to_path_buf();
    let files = extract(url, &archive, &out_dir).await?;
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    assert_eq!(1_023, files.len());
    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    assert_eq!(1_021, files.len());
    #[cfg(all(target_os = "macos", target_arch = "x86_64"))]
    assert_eq!(1_021, files.len());
    #[cfg(all(target_os = "windows", target_arch = "x86_64"))]
    assert_eq!(1_021, files.len());
    remove_dir_all(&out_dir)?;
    Ok(())
}

#[test(tokio::test)]
#[cfg(feature = "zonky")]
async fn test_get_archive_version_not_found() -> postgresql_archive::Result<()> {
    let invalid_version_req = VersionReq::parse("=1.0.0")?;
    let result = get_archive(zonky::URL, &invalid_version_req).await;

    assert!(result.is_err());
    Ok(())
}
