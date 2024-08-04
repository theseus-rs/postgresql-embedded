use crate::Error::Unexpected;
use crate::Result;
use human_bytes::human_bytes;
use num_format::{Locale, ToFormattedString};
use std::fs::{create_dir_all, remove_dir_all, remove_file, rename, File};
use std::io::{copy, BufReader, Cursor};
use std::path::{Path, PathBuf};
use std::thread::sleep;
use std::time::Duration;
use tar::Archive;
use tracing::{debug, instrument, warn};
use xz2::bufread::XzDecoder;
use zip::ZipArchive;

/// Extracts the compressed tar `bytes` to the [out_dir](Path).
///
/// # Errors
/// Returns an error if the extraction fails.
#[allow(clippy::case_sensitive_file_extension_comparisons)]
#[allow(clippy::cast_precision_loss)]
#[instrument(skip(bytes))]
pub fn extract(bytes: &Vec<u8>, out_dir: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
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
        return Ok(files);
    }

    let extract_dir = tempfile::tempdir_in(parent_dir)?.into_path();
    debug!("Extracting archive to {}", extract_dir.to_string_lossy());

    let reader = Cursor::new(bytes);
    let mut archive = ZipArchive::new(reader).map_err(|error| Unexpected(error.to_string()))?;
    let mut archive_bytes = Vec::new();
    for i in 0..archive.len() {
        let mut file = archive
            .by_index(i)
            .map_err(|error| Unexpected(error.to_string()))?;
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

    let input = BufReader::new(Cursor::new(archive_bytes));
    let decoder = XzDecoder::new(input);
    let mut archive = Archive::new(decoder);
    let mut extracted_bytes = 0;

    for archive_entry in archive.entries()? {
        let mut entry = archive_entry?;
        let entry_header = entry.header();
        let entry_type = entry_header.entry_type();
        let entry_size = entry_header.size()?;
        #[cfg(unix)]
        let file_mode = entry_header.mode()?;

        let entry_header_path = entry_header.path()?.to_path_buf();
        let mut entry_name = extract_dir.clone();
        entry_name.push(entry_header_path);

        if let Some(parent) = entry_name.parent() {
            if !parent.exists() {
                create_dir_all(parent)?;
            }
        }

        if entry_type.is_dir() || entry_name.is_dir() {
            create_dir_all(&entry_name)?;
        } else if entry_type.is_file() {
            let mut output_file = File::create(&entry_name)?;
            copy(&mut entry, &mut output_file)?;
            extracted_bytes += entry_size;

            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                output_file.set_permissions(std::fs::Permissions::from_mode(file_mode))?;
            }
            files.push(entry_name);
        } else if entry_type.is_symlink() {
            #[cfg(unix)]
            if let Some(symlink_target) = entry.link_name()? {
                let symlink_path = entry_name.clone();
                std::os::unix::fs::symlink(symlink_target.as_ref(), symlink_path)?;
                files.push(entry_name);
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

    let number_of_files = files.len();
    debug!(
        "Extracting {} files totalling {}",
        number_of_files.to_formatted_string(&Locale::en),
        human_bytes(extracted_bytes as f64)
    );

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
