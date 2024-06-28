#[allow(deprecated)]
use postgresql_archive::extract;
use postgresql_archive::{get_archive, get_version, DEFAULT_POSTGRESQL_URL};
use semver::VersionReq;
use std::fs::{create_dir_all, remove_dir_all};
use test_log::test;

async fn test_get_archive_for_version_constant(major: u64) -> anyhow::Result<()> {
    let version_req = VersionReq::parse(&format!("={major}"))?;
    let (archive_version, _archive) = get_archive(DEFAULT_POSTGRESQL_URL, &version_req).await?;

    assert!(version_req.matches(&archive_version));
    assert_eq!(major, archive_version.major);
    Ok(())
}

#[test(tokio::test)]
async fn test_get_archive_for_version_constant_v16() -> anyhow::Result<()> {
    test_get_archive_for_version_constant(16).await
}

#[test(tokio::test)]
async fn test_get_archive_for_version_constant_v15() -> anyhow::Result<()> {
    test_get_archive_for_version_constant(15).await
}

#[test(tokio::test)]
async fn test_get_archive_for_version_constant_v14() -> anyhow::Result<()> {
    test_get_archive_for_version_constant(14).await
}

#[test(tokio::test)]
async fn test_get_archive_for_version_constant_v13() -> anyhow::Result<()> {
    test_get_archive_for_version_constant(13).await
}

#[test(tokio::test)]
#[allow(deprecated)]
async fn test_get_archive_for_version_constant_v12() -> anyhow::Result<()> {
    test_get_archive_for_version_constant(12).await
}

#[test(tokio::test)]
async fn test_get_version_not_found() -> postgresql_archive::Result<()> {
    let invalid_version_req = VersionReq::parse("=1.0.0")?;
    let result = get_version(DEFAULT_POSTGRESQL_URL, &invalid_version_req).await;

    assert!(result.is_err());
    Ok(())
}

#[test(tokio::test)]
async fn test_get_version() -> anyhow::Result<()> {
    let version_req = VersionReq::STAR;
    let latest_version = get_version(DEFAULT_POSTGRESQL_URL, &version_req).await?;

    assert!(version_req.matches(&latest_version));
    Ok(())
}

#[test(tokio::test)]
async fn test_get_archive_and_extract() -> anyhow::Result<()> {
    let version_req = VersionReq::STAR;
    let (archive_version, archive) = get_archive(DEFAULT_POSTGRESQL_URL, &version_req).await?;

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
    let result = get_archive(DEFAULT_POSTGRESQL_URL, &invalid_version_req).await;

    assert!(result.is_err());
    Ok(())
}
