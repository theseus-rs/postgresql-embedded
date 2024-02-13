#[cfg(feature = "blocking")]
use postgresql_archive::blocking::{extract, get_archive, get_archive_for_target, get_version};
#[cfg(feature = "blocking")]
use postgresql_archive::LATEST;
#[cfg(feature = "blocking")]
use std::fs::{create_dir_all, remove_dir_all};
#[cfg(feature = "blocking")]
use test_log::test;

#[cfg(feature = "blocking")]
#[test]
fn test_get_version() -> anyhow::Result<()> {
    let version = &LATEST;

    assert!(version.minor.is_none());
    assert!(version.release.is_none());

    let latest_version = get_version(version)?;

    assert_eq!(version.major, latest_version.major);
    assert!(latest_version.minor.is_some());
    assert!(latest_version.release.is_some());

    Ok(())
}

#[cfg(feature = "blocking")]
#[test]
#[allow(deprecated)]
fn test_get_archive_and_extract() -> anyhow::Result<()> {
    let version = &LATEST;
    let (archive_version, archive) = get_archive(version)?;

    assert!(archive_version.matches(version));

    let out_dir = tempfile::tempdir()?.path().to_path_buf();
    create_dir_all(&out_dir)?;
    extract(&archive, &out_dir)?;
    remove_dir_all(&out_dir)?;

    Ok(())
}

#[cfg(feature = "blocking")]
#[test]
#[allow(deprecated)]
fn test_get_archive_for_target() -> anyhow::Result<()> {
    let version = &LATEST;
    let (archive_version, _archive) = get_archive_for_target(version, target_triple::TARGET)?;

    assert!(archive_version.matches(version));

    Ok(())
}
