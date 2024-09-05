use crate::extractor::ExtractDirectories;
use crate::Result;
use human_bytes::human_bytes;
use num_format::{Locale, ToFormattedString};
use std::fs::create_dir_all;
use std::io::Cursor;
use std::path::PathBuf;
use std::{fs, io};
use tracing::{debug, instrument, warn};
use zip::ZipArchive;

/// Extracts the compressed tar `bytes` to paths defined in `extract_directories`.
///
/// # Errors
/// Returns an error if the extraction fails.
#[expect(clippy::cast_precision_loss)]
#[instrument(skip(bytes))]
pub fn extract(bytes: &Vec<u8>, extract_directories: ExtractDirectories) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    let reader = Cursor::new(bytes);
    let mut archive =
        ZipArchive::new(reader).map_err(|_| io::Error::new(io::ErrorKind::Other, "Zip error"))?;
    let mut extracted_bytes = 0;

    for i in 0..archive.len() {
        let mut file = archive
            .by_index(i)
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "Zip error"))?;
        let file_path = PathBuf::from(file.name());
        let file_path = PathBuf::from(file_path.file_name().unwrap_or_default());
        let file_name = file_path.to_string_lossy();

        let Ok(extract_dir) = extract_directories.get_path(&file_name) else {
            continue;
        };
        create_dir_all(&extract_dir)?;

        let mut out = Vec::new();
        io::copy(&mut file, &mut out)?;
        extracted_bytes += out.len() as u64;
        let path = PathBuf::from(&extract_dir).join(file_path);
        fs::write(&path, out)?;
        files.push(path);
    }

    let number_of_files = files.len();
    debug!(
        "Extracted {} files totalling {}",
        number_of_files.to_formatted_string(&Locale::en),
        human_bytes(extracted_bytes as f64)
    );

    Ok(files)
}
