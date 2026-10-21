#![cfg(any(feature = "tar-gz", feature = "tar-xz"))]

use postgresql_archive::extractor::ExtractDirectories;
use regex_lite::Regex;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use tar::{Builder, EntryType, Header};

#[derive(Clone, Copy)]
enum Format {
    #[cfg(feature = "tar-gz")]
    Gzip,
    #[cfg(feature = "tar-xz")]
    Xz,
}

const FORMATS: &[Format] = &[
    #[cfg(feature = "tar-gz")]
    Format::Gzip,
    #[cfg(feature = "tar-xz")]
    Format::Xz,
];

impl Format {
    fn extract(
        self,
        archive: &[u8],
        directories: &ExtractDirectories,
    ) -> anyhow::Result<Vec<PathBuf>> {
        match self {
            #[cfg(feature = "tar-gz")]
            Self::Gzip => {
                let mut encoder =
                    flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
                encoder.write_all(archive)?;
                Ok(postgresql_archive::extractor::tar_gz_extract(
                    &encoder.finish()?,
                    directories,
                )?)
            }
            #[cfg(feature = "tar-xz")]
            Self::Xz => {
                let mut encoder = liblzma::write::XzEncoder::new(Vec::new(), 1);
                encoder.write_all(archive)?;
                Ok(postgresql_archive::extractor::tar_xz_extract(
                    &encoder.finish()?,
                    directories,
                )?)
            }
        }
    }

    fn path(self, root: &Path, archive_path: &str) -> anyhow::Result<PathBuf> {
        let path = Path::new(archive_path);
        match self {
            #[cfg(feature = "tar-gz")]
            Self::Gzip => Ok(root.join(path.strip_prefix("postgresql")?)),
            #[cfg(feature = "tar-xz")]
            Self::Xz => Ok(root.join(path)),
        }
    }
}

fn archive() -> anyhow::Result<Builder<Vec<u8>>> {
    let mut archive = Builder::new(Vec::new());
    for path in [
        "postgresql",
        "postgresql/share",
        "postgresql/share/timezone",
    ] {
        let mut header = Header::new_gnu();
        header.set_entry_type(EntryType::Directory);
        header.set_mode(0o755);
        header.set_size(0);
        archive.append_data(&mut header, path, &[][..])?;
    }
    Ok(archive)
}

fn file(archive: &mut Builder<Vec<u8>>, path: &str) -> anyhow::Result<()> {
    let contents = b"timezone data";
    let mut header = Header::new_gnu();
    header.set_mode(0o644);
    header.set_size(contents.len() as u64);
    archive.append_data(&mut header, path, &contents[..])?;
    Ok(())
}

fn link(
    archive: &mut Builder<Vec<u8>>,
    path: &str,
    target: &str,
    entry_type: EntryType,
) -> anyhow::Result<()> {
    let mut header = Header::new_gnu();
    header.set_entry_type(entry_type);
    header.set_mode(0o644);
    header.set_size(0);
    archive.append_link(&mut header, path, target)?;
    Ok(())
}

fn directories(root: &Path) -> anyhow::Result<ExtractDirectories> {
    Ok(ExtractDirectories::new(vec![(
        Regex::new("^postgresql$")?,
        root.to_path_buf(),
    )]))
}

#[test]
fn test_timezone_hard_links() -> anyhow::Result<()> {
    for format in FORMATS {
        let root = tempfile::tempdir()?;
        let mut archive = archive()?;
        // Both forward references and chains must work independently of entry order.
        link(
            &mut archive,
            "postgresql/share/timezone/Etc/UTC",
            "postgresql/share/timezone/UTC",
            EntryType::Link,
        )?;
        link(
            &mut archive,
            "postgresql/share/timezone/UTC",
            "postgresql/share/timezone/Universal",
            EntryType::Link,
        )?;
        file(&mut archive, "postgresql/share/timezone/Universal")?;
        link(
            &mut archive,
            "postgresql/share/timezone/Zulu",
            "postgresql/share/timezone/Universal",
            EntryType::Link,
        )?;

        let files = format.extract(&archive.into_inner()?, &directories(root.path())?)?;
        assert_eq!(files.len(), 4);
        for name in ["Universal", "UTC", "Etc/UTC", "Zulu"] {
            let path = format.path(root.path(), &format!("postgresql/share/timezone/{name}"))?;
            assert_eq!(fs::read(&path)?, b"timezone data");
            assert!(files.contains(&path));
            #[cfg(unix)]
            {
                use std::os::unix::fs::MetadataExt;
                let source = format.path(root.path(), "postgresql/share/timezone/Universal")?;
                assert_eq!(fs::metadata(path)?.ino(), fs::metadata(source)?.ino());
            }
        }
    }
    Ok(())
}

#[test]
fn test_hard_link_targets_use_directory_mappings() -> anyhow::Result<()> {
    for format in FORMATS {
        let root = tempfile::tempdir()?;
        let source = root.path().join("source");
        let destination = root.path().join("destination");
        let mut directories = directories(&destination)?;
        directories.add_mapping(Regex::new("^data$")?, source.clone());
        let mut archive = archive()?;
        let mut header = Header::new_gnu();
        header.set_entry_type(EntryType::Directory);
        header.set_mode(0o755);
        header.set_size(0);
        archive.append_data(&mut header, "data", &[][..])?;
        file(&mut archive, "data/Universal")?;
        link(
            &mut archive,
            "postgresql/share/timezone/UTC",
            "data/Universal",
            EntryType::Link,
        )?;

        let files = format.extract(&archive.into_inner()?, &directories)?;
        let utc = format.path(&destination, "postgresql/share/timezone/UTC")?;
        assert_eq!(fs::read(&utc)?, b"timezone data");
        assert!(files.contains(&utc));
    }
    Ok(())
}

#[test]
fn test_missing_and_cyclic_hard_link_targets_fail() -> anyhow::Result<()> {
    for format in FORMATS {
        for cyclic in [false, true] {
            let root = tempfile::tempdir()?;
            let mut archive = archive()?;
            link(
                &mut archive,
                "postgresql/share/timezone/UTC",
                "postgresql/share/timezone/Universal",
                EntryType::Link,
            )?;
            if cyclic {
                link(
                    &mut archive,
                    "postgresql/share/timezone/Universal",
                    "postgresql/share/timezone/UTC",
                    EntryType::Link,
                )?;
            }
            let result = format.extract(&archive.into_inner()?, &directories(root.path())?);
            assert!(result.is_err());
        }
    }
    Ok(())
}

#[test]
fn test_hard_link_header_without_target_fails() -> anyhow::Result<()> {
    for format in FORMATS {
        let root = tempfile::tempdir()?;
        let mut archive = archive()?;
        let mut header = Header::new_gnu();
        header.set_entry_type(EntryType::Link);
        header.set_mode(0o644);
        header.set_size(0);
        archive.append_data(&mut header, "postgresql/share/timezone/UTC", &[][..])?;

        let error = format
            .extract(&archive.into_inner()?, &directories(root.path())?)
            .expect_err("a hard link must name its target");
        assert!(error.to_string().contains("Missing hard link target"));
        assert!(
            !format
                .path(root.path(), "postgresql/share/timezone/UTC")?
                .exists()
        );
    }
    Ok(())
}

#[test]
fn test_hard_link_targets_cannot_escape_destination() -> anyhow::Result<()> {
    for format in FORMATS {
        let root = tempfile::tempdir()?;
        let outside = root.path().join("outside");
        fs::write(&outside, b"outside data")?;
        for target in [
            outside.to_string_lossy().into_owned(),
            "postgresql/../outside".to_string(),
            "postgresql/share/../../outside".to_string(),
        ] {
            let destination = tempfile::tempdir_in(root.path())?;
            let mut archive = archive()?;
            link(
                &mut archive,
                "postgresql/share/timezone/UTC",
                &target,
                EntryType::Link,
            )?;
            let result = format.extract(&archive.into_inner()?, &directories(destination.path())?);
            assert!(result.is_err());
            assert_eq!(fs::read(&outside)?, b"outside data");
            assert!(
                !format
                    .path(destination.path(), "postgresql/share/timezone/UTC")?
                    .exists()
            );
        }
    }
    Ok(())
}

#[cfg(unix)]
#[test]
fn test_hard_links_cannot_follow_symlinks_outside_destination() -> anyhow::Result<()> {
    for format in FORMATS {
        for link_is_target in [false, true] {
            let root = tempfile::tempdir()?;
            let destination = root.path().join("destination");
            let outside = root.path().join("outside");
            fs::create_dir(&outside)?;
            fs::write(outside.join("Universal"), b"outside data")?;
            let mut archive = archive()?;
            file(&mut archive, "postgresql/share/timezone/Universal")?;
            link(
                &mut archive,
                "postgresql/external",
                &outside.to_string_lossy(),
                EntryType::Symlink,
            )?;
            let (path, target) = if link_is_target {
                (
                    "postgresql/share/timezone/UTC",
                    "postgresql/external/Universal",
                )
            } else {
                (
                    "postgresql/external/UTC",
                    "postgresql/share/timezone/Universal",
                )
            };
            link(&mut archive, path, target, EntryType::Link)?;
            let result = format.extract(&archive.into_inner()?, &directories(&destination)?);
            assert!(result.is_err());
            assert!(!outside.join("UTC").exists());
        }
    }
    Ok(())
}
