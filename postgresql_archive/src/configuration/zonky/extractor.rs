use crate::Error::Unexpected;
use crate::Result;
use crate::extractor::{ExtractDirectories, tar_xz_extract};
use regex_lite::Regex;
use std::fs::{create_dir_all, remove_dir_all, remove_file, rename};
use std::io::Cursor;
use std::path::{Path, PathBuf};
use std::thread::sleep;
use std::time::Duration;
use tracing::{debug, instrument, warn};
use zip::ZipArchive;

/// Extracts the compressed tar `bytes` to the [out_dir](Path).
///
/// # Errors
/// Returns an error if the extraction fails.
#[expect(clippy::case_sensitive_file_extension_comparisons)]
#[instrument(skip(bytes))]
pub fn extract(bytes: &Vec<u8>, extract_directories: &ExtractDirectories) -> Result<Vec<PathBuf>> {
    let out_dir = extract_directories.get_path(".")?;
    let parent_dir = if let Some(parent) = out_dir.parent() {
        parent
    } else {
        debug!("No parent directory for {}", out_dir.to_string_lossy());
        out_dir.as_path()
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
        return Ok(Vec::new());
    }

    let extract_dir = tempfile::tempdir_in(parent_dir)?.keep();
    debug!("Extracting archive to {}", extract_dir.to_string_lossy());

    let reader = Cursor::new(bytes);
    let mut archive = ZipArchive::new(reader)?;
    let mut archive_bytes = Vec::new();
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let file_name = file.name().to_string();
        if file_name.ends_with(".txz") {
            debug!("Found archive file: {file_name}");
            std::io::copy(&mut file, &mut archive_bytes)?;
            break;
        }
    }

    if archive_bytes.is_empty() {
        return Err(Unexpected("Failed to find archive file".to_string()));
    }

    let mut archive_extract_directories = ExtractDirectories::default();
    archive_extract_directories.add_mapping(Regex::new(".*")?, extract_dir.clone());
    let files = tar_xz_extract(&archive_bytes, &archive_extract_directories)?;

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

    Ok(files)
}

/// Acquires a lock file in the [out_dir](Path) to prevent multiple processes from extracting the
/// archive at the same time.
///
/// # Errors
/// * If the lock file cannot be acquired.
#[instrument(level = "debug")]
fn acquire_lock(out_dir: &Path) -> crate::Result<PathBuf> {
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
