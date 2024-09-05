use crate::extractor::ExtractDirectories;
use crate::Error::Unexpected;
use crate::Result;
use human_bytes::human_bytes;
use liblzma::bufread::XzDecoder;
use num_format::{Locale, ToFormattedString};
use std::fs::{create_dir_all, File};
use std::io::{copy, BufReader, Cursor};
use std::path::PathBuf;
use tar::Archive;
use tracing::{debug, instrument, warn};

/// Extracts the compressed tar `bytes` to paths defined in `extract_directories`.
///
/// # Errors
/// Returns an error if the extraction fails.
#[expect(clippy::cast_precision_loss)]
#[instrument(skip(bytes))]
pub fn extract(bytes: &Vec<u8>, extract_directories: ExtractDirectories) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    let input = BufReader::new(Cursor::new(bytes));
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
        let prefix = match entry_header_path.components().next() {
            Some(component) => component.as_os_str().to_str().unwrap_or_default(),
            None => {
                return Err(Unexpected(
                    "Failed to get file header path prefix".to_string(),
                ));
            }
        };
        let stripped_entry_header_path = entry_header_path.strip_prefix(prefix)?.to_path_buf();
        let Ok(extract_dir) = extract_directories.get_path(prefix) else {
            continue;
        };
        let mut entry_name = extract_dir.clone();
        entry_name.push(stripped_entry_header_path);

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

    let number_of_files = files.len();
    debug!(
        "Extracted {} files totalling {}",
        number_of_files.to_formatted_string(&Locale::en),
        human_bytes(extracted_bytes as f64)
    );

    Ok(files)
}
