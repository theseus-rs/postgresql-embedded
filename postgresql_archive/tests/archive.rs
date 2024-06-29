#[allow(deprecated)]
use postgresql_archive::extract;
use postgresql_archive::{get_archive, get_version, THESEUS_POSTGRESQL_BINARIES_URL};
use semver::VersionReq;
use std::fs::{create_dir_all, remove_dir_all};
use test_log::test;

#[test(tokio::test)]
async fn test_get_version_not_found() -> postgresql_archive::Result<()> {
    let invalid_version_req = VersionReq::parse("=1.0.0")?;
    let result = get_version(THESEUS_POSTGRESQL_BINARIES_URL, &invalid_version_req).await;

    assert!(result.is_err());
    Ok(())
}

#[test(tokio::test)]
async fn test_get_version() -> anyhow::Result<()> {
    let version_req = VersionReq::parse("=16.3.0")?;
    let latest_version = get_version(THESEUS_POSTGRESQL_BINARIES_URL, &version_req).await?;

    assert!(version_req.matches(&latest_version));
    Ok(())
}

#[test(tokio::test)]
async fn test_get_archive_and_extract() -> anyhow::Result<()> {
    let version_req = VersionReq::STAR;
    let (archive_version, archive) =
        get_archive(THESEUS_POSTGRESQL_BINARIES_URL, &version_req).await?;

    assert!(version_req.matches(&archive_version));

    let out_dir = tempfile::tempdir()?.path().to_path_buf();
    create_dir_all(&out_dir)?;
    extract(&archive, &out_dir).await?;
    remove_dir_all(&out_dir)?;
    Ok(())
}

#[test(tokio::test)]
async fn test_get_archive_version_not_found() -> postgresql_archive::Result<()> {
    let invalid_version_req = VersionReq::parse("=1.0.0")?;
    let result = get_archive(THESEUS_POSTGRESQL_BINARIES_URL, &invalid_version_req).await;

    assert!(result.is_err());
    Ok(())
}
