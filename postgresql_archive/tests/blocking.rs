#[cfg(feature = "blocking")]
use postgresql_archive::blocking::{extract, get_archive, get_version};
#[cfg(feature = "blocking")]
use postgresql_archive::configuration::theseus;
#[cfg(feature = "blocking")]
use postgresql_archive::VersionReq;
#[cfg(feature = "blocking")]
use std::fs::remove_dir_all;
#[cfg(feature = "blocking")]
use test_log::test;

#[cfg(feature = "blocking")]
#[test]
fn test_get_version() -> anyhow::Result<()> {
    let version_req = VersionReq::STAR;
    let latest_version = get_version(theseus::URL, &version_req)?;

    assert!(version_req.matches(&latest_version));
    Ok(())
}

#[cfg(feature = "blocking")]
#[test]
fn test_get_archive_and_extract() -> anyhow::Result<()> {
    let url = theseus::URL;
    let version_req = &VersionReq::parse("=16.4.0")?;
    let (archive_version, archive) = get_archive(url, version_req)?;

    assert!(version_req.matches(&archive_version));

    let out_dir = tempfile::tempdir()?.path().to_path_buf();
    let files = extract(url, &archive, &out_dir)?;
    assert!(!files.is_empty());
    remove_dir_all(&out_dir)?;
    Ok(())
}
