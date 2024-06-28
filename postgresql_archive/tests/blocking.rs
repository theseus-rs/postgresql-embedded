#[cfg(feature = "blocking")]
use postgresql_archive::blocking::{extract, get_archive, get_version};
#[cfg(feature = "blocking")]
use postgresql_archive::{VersionReq, DEFAULT_POSTGRESQL_URL};
#[cfg(feature = "blocking")]
use std::fs::{create_dir_all, remove_dir_all};
#[cfg(feature = "blocking")]
use test_log::test;

#[cfg(feature = "blocking")]
#[test]
fn test_get_version() -> anyhow::Result<()> {
    let version_req = VersionReq::STAR;
    let latest_version = get_version(DEFAULT_POSTGRESQL_URL, &version_req)?;

    assert!(version_req.matches(&latest_version));
    Ok(())
}

#[cfg(feature = "blocking")]
#[test]
#[allow(deprecated)]
fn test_get_archive_and_extract() -> anyhow::Result<()> {
    let version_req = &VersionReq::STAR;
    let (archive_version, archive) = get_archive(DEFAULT_POSTGRESQL_URL, version_req)?;

    assert!(version_req.matches(&archive_version));

    let out_dir = tempfile::tempdir()?.path().to_path_buf();
    create_dir_all(&out_dir)?;
    extract(&archive, &out_dir)?;
    remove_dir_all(&out_dir)?;
    Ok(())
}
