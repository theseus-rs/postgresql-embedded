#[allow(deprecated)]
use postgresql_archive::{extract, Version, LATEST, V12, V13, V14, V15, V16};
use postgresql_archive::{get_archive, get_archive_for_target, get_version};
use std::fs::{create_dir_all, remove_dir_all};
use test_log::test;

async fn test_get_archive_for_version_constant(version: Version) -> anyhow::Result<()> {
    let (_archive_version, _archive) = get_archive(&version).await?;
    Ok(())
}

#[test(tokio::test)]
async fn test_get_archive_for_version_constant_v16() -> anyhow::Result<()> {
    test_get_archive_for_version_constant(V16).await
}

#[test(tokio::test)]
async fn test_get_archive_for_version_constant_v15() -> anyhow::Result<()> {
    test_get_archive_for_version_constant(V15).await
}

#[test(tokio::test)]
async fn test_get_archive_for_version_constant_v14() -> anyhow::Result<()> {
    test_get_archive_for_version_constant(V14).await
}

#[test(tokio::test)]
async fn test_get_archive_for_version_constant_v13() -> anyhow::Result<()> {
    test_get_archive_for_version_constant(V13).await
}

#[test(tokio::test)]
#[allow(deprecated)]
async fn test_get_archive_for_version_constant_v12() -> anyhow::Result<()> {
    test_get_archive_for_version_constant(V12).await
}

#[test(tokio::test)]
async fn test_get_version_not_found() -> postgresql_archive::Result<()> {
    let invalid_version = Version::new(1, Some(0), Some(0));
    let result = get_version(&invalid_version).await;
    assert!(result.is_err());
    Ok(())
}

#[test(tokio::test)]
async fn test_get_version() -> anyhow::Result<()> {
    let version = &LATEST;

    assert!(version.minor.is_none());
    assert!(version.release.is_none());

    let latest_version = get_version(version).await?;

    assert_eq!(version.major, latest_version.major);
    assert!(latest_version.minor.is_some());
    assert!(latest_version.release.is_some());

    Ok(())
}

#[test(tokio::test)]
async fn test_get_archive_and_extract() -> anyhow::Result<()> {
    let version = &LATEST;
    let (archive_version, archive) = get_archive(version).await?;

    assert!(archive_version.matches(version));

    let out_dir = tempfile::tempdir()?.path().to_path_buf();
    create_dir_all(&out_dir)?;
    extract(&archive, &out_dir).await?;
    remove_dir_all(&out_dir)?;

    Ok(())
}

#[test(tokio::test)]
async fn test_get_archive_version_not_found() -> postgresql_archive::Result<()> {
    let invalid_version = Version::new(1, Some(0), Some(0));
    let result = get_archive(&invalid_version).await;
    assert!(result.is_err());
    Ok(())
}

#[test(tokio::test)]
async fn test_get_archive_for_target_version_not_found() -> postgresql_archive::Result<()> {
    let invalid_version = Version::new(1, Some(0), Some(0));
    let result = get_archive_for_target(&invalid_version, target_triple::TARGET).await;
    assert!(result.is_err());
    Ok(())
}

#[test(tokio::test)]
async fn test_get_archive_for_target_target_not_found() -> postgresql_archive::Result<()> {
    let result = get_archive_for_target(&LATEST, "wasm64-unknown-unknown").await;
    assert!(result.is_err());
    Ok(())
}

#[test(tokio::test)]
async fn test_get_archive_for_target() -> anyhow::Result<()> {
    let version = &LATEST;
    let (archive_version, _archive) =
        get_archive_for_target(version, target_triple::TARGET).await?;

    assert!(archive_version.matches(version));

    Ok(())
}
