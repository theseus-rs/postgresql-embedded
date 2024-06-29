//! Manage PostgreSQL archives
#![allow(dead_code)]

use crate::error::Error::Unexpected;
use crate::error::Result;
use crate::repository;
use bytes::Bytes;
use flate2::bufread::GzDecoder;
use human_bytes::human_bytes;
use num_format::{Locale, ToFormattedString};
use semver::{Version, VersionReq};
use std::fs::{create_dir_all, remove_dir_all, remove_file, rename, File};
use std::io::{copy, BufReader, Cursor};
use std::path::{Path, PathBuf};
use std::thread::sleep;
use std::time::Duration;
use tar::Archive;
use tracing::{debug, instrument, warn};

pub const DEFAULT_POSTGRESQL_URL: &str = "https://github.com/theseus-rs/postgresql-binaries";

/// Gets the version for the specified [version requirement](VersionReq). If a version for the
/// [version requirement](VersionReq) is not found, then an error is returned.
///
/// # Errors
/// * If the version is not found.
#[instrument(level = "debug")]
pub async fn get_version(url: &str, version_req: &VersionReq) -> Result<Version> {
    let repository = repository::registry::get(url)?;
    let version = repository.get_version(version_req).await?;
    Ok(version)
}

/// Gets the archive for a given [version requirement](VersionReq) that passes the default
/// matcher. If no archive is found for the [version requirement](VersionReq) and matcher then
/// an [error](crate::error::Error) is returned.
///
/// # Errors
/// * If the archive is not found.
/// * If the archive cannot be downloaded.
#[instrument]
pub async fn get_archive(url: &str, version_req: &VersionReq) -> Result<(Version, Bytes)> {
    let repository = repository::registry::get(url)?;
    let archive = repository.get_archive(version_req).await?;
    let version = archive.version().clone();
    let archive_bytes = archive.bytes().to_vec();
    let bytes = Bytes::from(archive_bytes.clone());
    Ok((version, bytes))
}

/// Acquires a lock file in the [out_dir](Path) to prevent multiple processes from extracting the
/// archive at the same time.
///
/// # Errors
/// * If the lock file cannot be acquired.
#[instrument(level = "debug")]
fn acquire_lock(out_dir: &Path) -> Result<PathBuf> {
    let lock_file = out_dir.join("postgresql-archive.lock");

    if lock_file.is_file() {
        let metadata = lock_file.metadata()?;
        let created = metadata.created()?;

        if created.elapsed()?.as_secs() > 300 {
            warn!(
                "Stale lock file detected; removing file to attempt process recovery: {}",
                lock_file.to_string_lossy()
            );
            remove_file(&lock_file)?;
        }
    }

    debug!(
        "Attempting to acquire lock: {}",
        lock_file.to_string_lossy()
    );

    for _ in 0..30 {
        let lock = std::fs::OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(&lock_file);

        match lock {
            Ok(_) => {
                debug!("Lock acquired: {}", lock_file.to_string_lossy());
                return Ok(lock_file);
            }
            Err(error) => {
                warn!("unable to acquire lock: {error}");
                sleep(Duration::from_secs(1));
            }
        }
    }

    Err(Unexpected("Failed to acquire lock".to_string()))
}

/// Extracts the compressed tar [bytes](Bytes) to the [out_dir](Path).
///
/// # Errors
/// Returns an error if the extraction fails.
#[allow(clippy::cast_precision_loss)]
#[instrument(skip(bytes))]
pub async fn extract(bytes: &Bytes, out_dir: &Path) -> Result<()> {
    let input = BufReader::new(Cursor::new(bytes));
    let decoder = GzDecoder::new(input);
    let mut archive = Archive::new(decoder);
    let mut files = 0;
    let mut extracted_bytes = 0;

    let parent_dir = if let Some(parent) = out_dir.parent() {
        parent
    } else {
        debug!("No parent directory for {}", out_dir.to_string_lossy());
        out_dir
    };

    create_dir_all(parent_dir)?;

    let lock_file = acquire_lock(parent_dir)?;
    // If the directory already exists, then the archive has already been
    // extracted by another process.
    if out_dir.exists() {
        debug!(
            "Directory already exists {}; skipping extraction: ",
            out_dir.to_string_lossy()
        );
        remove_file(&lock_file)?;
        return Ok(());
    }

    let extract_dir = tempfile::tempdir_in(parent_dir)?.into_path();
    debug!("Extracting archive to {}", extract_dir.to_string_lossy());

    for archive_entry in archive.entries()? {
        let mut entry = archive_entry?;
        let entry_header = entry.header();
        let entry_type = entry_header.entry_type();
        let entry_size = entry_header.size()?;
        #[cfg(unix)]
        let file_mode = entry_header.mode()?;

        let entry_header_path = entry_header.path()?.to_path_buf();
        let prefix = match entry_header_path.components().next() {
            Some(component) => component.as_os_str().to_str().unwrap_or_default(),
            None => {
                return Err(Unexpected(
                    "Failed to get file header path prefix".to_string(),
                ));
            }
        };
        let stripped_entry_header_path = entry_header_path.strip_prefix(prefix)?.to_path_buf();
        let mut entry_name = extract_dir.clone();
        entry_name.push(stripped_entry_header_path);

        if entry_type.is_dir() || entry_name.is_dir() {
            create_dir_all(&entry_name)?;
        } else if entry_type.is_file() {
            let mut output_file = File::create(&entry_name)?;
            copy(&mut entry, &mut output_file)?;

            files += 1;
            extracted_bytes += entry_size;

            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                output_file.set_permissions(std::fs::Permissions::from_mode(file_mode))?;
            }
        } else if entry_type.is_symlink() {
            #[cfg(unix)]
            if let Some(symlink_target) = entry.link_name()? {
                let symlink_path = entry_name;
                std::os::unix::fs::symlink(symlink_target.as_ref(), symlink_path)?;
            }
        }
    }

    if out_dir.exists() {
        debug!(
            "Directory already exists {}; skipping rename and removing extraction directory: {}",
            out_dir.to_string_lossy(),
            extract_dir.to_string_lossy()
        );
        remove_dir_all(&extract_dir)?;
    } else {
        debug!(
            "Renaming {} to {}",
            extract_dir.to_string_lossy(),
            out_dir.to_string_lossy()
        );
        rename(extract_dir, out_dir)?;
    }

    if lock_file.is_file() {
        debug!("Removing lock file: {}", lock_file.to_string_lossy());
        remove_file(lock_file)?;
    }

    debug!(
        "Extracting {} files totalling {}",
        files.to_formatted_string(&Locale::en),
        human_bytes(extracted_bytes as f64)
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_version() -> Result<()> {
        let version_req = VersionReq::parse("=16.3.0")?;
        let version = get_version(DEFAULT_POSTGRESQL_URL, &version_req).await?;
        assert_eq!(Version::new(16, 3, 0), version);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_archive() -> Result<()> {
        let version_req = VersionReq::parse("=16.3.0")?;
        let (version, bytes) = get_archive(DEFAULT_POSTGRESQL_URL, &version_req).await?;
        assert_eq!(Version::new(16, 3, 0), version);
        assert!(!bytes.is_empty());
        Ok(())
    }
}
