#[allow(deprecated)]
use postgresql_archive::{extract, Version, LATEST, V12, V13, V14, V15, V16};
use postgresql_archive::{get_archive, get_archive_for_target, get_version};
use sha2::{Digest, Sha256};
use std::fs::{create_dir_all, remove_dir_all};

#[tokio::test]
#[allow(deprecated)]
async fn test_get_archive_for_version_constants() -> anyhow::Result<()> {
    let versions = vec![V12, V13, V14, V15, V16];

    for version in versions {
        let (_, archive, hash) = get_archive(&version).await?;

        let mut hasher = Sha256::new();
        hasher.update(&archive);
        let archive_hash = hex::encode(&hasher.finalize());

        assert_eq!(archive_hash, hash);
    }

    Ok(())
}

#[tokio::test]
async fn test_get_version_not_found() -> postgresql_archive::Result<()> {
    let invalid_version = Version::new(1, Some(0), Some(0));
    let result = get_version(&invalid_version).await;
    assert!(result.is_err());
    Ok(())
}

#[tokio::test]
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

#[tokio::test]
async fn test_get_archive_and_extract() -> anyhow::Result<()> {
    let version = &LATEST;
    let (archive_version, archive, hash) = get_archive(version).await?;

    let mut hasher = Sha256::new();
    hasher.update(&archive);
    let archive_hash = hex::encode(&hasher.finalize());

    assert_eq!(archive_hash, hash);
    assert!(archive_version.matches(version));

    let out_dir = tempfile::tempdir()?.path().to_path_buf();
    create_dir_all(&out_dir)?;
    extract(&archive, &out_dir).await?;
    remove_dir_all(&out_dir)?;

    Ok(())
}

#[tokio::test]
async fn test_get_archive_version_not_found() -> postgresql_archive::Result<()> {
    let invalid_version = Version::new(1, Some(0), Some(0));
    let result = get_archive(&invalid_version).await;
    assert!(result.is_err());
    Ok(())
}

#[tokio::test]
async fn test_get_archive_for_target_version_not_found() -> postgresql_archive::Result<()> {
    let invalid_version = Version::new(1, Some(0), Some(0));
    let result = get_archive_for_target(&invalid_version, target_triple::TARGET).await;
    assert!(result.is_err());
    Ok(())
}

#[tokio::test]
async fn test_get_archive_for_target_target_not_found() -> postgresql_archive::Result<()> {
    let result = get_archive_for_target(&LATEST, "wasm64-unknown-unknown").await;
    assert!(result.is_err());
    Ok(())
}

#[tokio::test]
async fn test_get_archive_for_target() -> anyhow::Result<()> {
    let version = &LATEST;
    let (archive_version, archive, hash) =
        get_archive_for_target(version, target_triple::TARGET).await?;

    let mut hasher = Sha256::new();
    hasher.update(&archive);
    let archive_hash = hex::encode(&hasher.finalize());

    assert_eq!(archive_hash, hash);
    assert!(archive_version.matches(version));

    Ok(())
}
