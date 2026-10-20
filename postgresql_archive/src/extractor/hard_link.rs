use crate::Error::Unexpected;
use crate::Result;
use crate::extractor::ExtractDirectories;
use std::fs;
use std::io::ErrorKind;
use std::path::{Component, Path, PathBuf};

/// A TAR hard link whose target may appear later in the archive.
#[derive(Debug)]
pub(super) struct HardLink {
    source: PathBuf,
    source_root: PathBuf,
    destination: PathBuf,
    destination_root: PathBuf,
}

impl HardLink {
    /// Map the archive-relative target just like a regular archive entry.
    pub(super) fn new(
        destination: PathBuf,
        destination_root: PathBuf,
        target: &Path,
        directories: &ExtractDirectories,
        strip_prefix: bool,
    ) -> Result<Self> {
        validate_relative_path(target)?;
        validate_relative_path(destination.strip_prefix(&destination_root)?)?;
        let prefix = target.components().next().ok_or_else(|| {
            Unexpected(format!(
                "Missing hard link target for {}",
                destination.display()
            ))
        })?;
        let prefix = prefix.as_os_str();
        let source_root = directories.get_path(prefix.to_str().unwrap_or_default())?;
        let target = if strip_prefix {
            target.strip_prefix(prefix)?
        } else {
            target
        };

        Ok(Self {
            source: source_root.join(target),
            source_root,
            destination,
            destination_root,
        })
    }

    /// Returns false when the target has not been extracted yet.
    fn extract(&self) -> Result<bool> {
        let metadata = match fs::symlink_metadata(&self.source) {
            Ok(metadata) => metadata,
            Err(error) if error.kind() == ErrorKind::NotFound => return Ok(false),
            Err(error) => return Err(error.into()),
        };
        let source_root = self.source_root.canonicalize()?;
        let source = self.source.canonicalize()?;
        if !metadata.is_file() || !source.starts_with(&source_root) {
            return Err(Unexpected(format!(
                "Hard link target is not a regular file within the extraction directory: {}",
                self.source.display()
            )));
        }

        // Validate each parent before descending, so an existing symlink cannot
        // cause directory or hard-link creation outside the configured root.
        fs::create_dir_all(&self.destination_root)?;
        let destination_root = self.destination_root.canonicalize()?;
        let mut parent = destination_root.clone();
        let relative = self.destination.strip_prefix(&self.destination_root)?;
        if let Some(relative_parent) = relative.parent() {
            for component in relative_parent.components() {
                parent.push(component);
                match fs::create_dir(&parent) {
                    Ok(()) => {}
                    Err(error) if error.kind() == ErrorKind::AlreadyExists => {}
                    Err(error) => return Err(error.into()),
                }
                parent = parent.canonicalize()?;
                if !parent.starts_with(&destination_root) {
                    return Err(Unexpected(format!(
                        "Hard link destination escapes the extraction directory: {}",
                        self.destination.display()
                    )));
                }
            }
        }
        let name = relative.file_name().ok_or_else(|| {
            Unexpected(format!(
                "Missing hard link file name: {}",
                self.destination.display()
            ))
        })?;
        let destination = parent.join(name);
        fs::hard_link(&source, &destination)?;
        Ok(true)
    }
}

/// Resolve forward references and chains, reporting missing or cyclic targets.
pub(super) fn extract_hard_links(
    mut pending: Vec<HardLink>,
    files: &mut Vec<PathBuf>,
) -> Result<()> {
    while !pending.is_empty() {
        let count = pending.len();
        let mut unresolved = Vec::new();
        for link in pending {
            if link.extract()? {
                files.push(link.destination);
            } else {
                unresolved.push(link);
            }
        }
        if unresolved.len() == count {
            let targets: Vec<_> = unresolved
                .iter()
                .map(|link| link.source.display().to_string())
                .collect();
            return Err(Unexpected(format!(
                "Missing or cyclic hard link targets: {}",
                targets.join(", ")
            )));
        }
        pending = unresolved;
    }
    Ok(())
}

fn validate_relative_path(path: &Path) -> Result<()> {
    if path.components().any(|component| {
        matches!(
            component,
            Component::ParentDir | Component::RootDir | Component::Prefix(_)
        )
    }) {
        return Err(Unexpected(format!(
            "Invalid hard link path: {}",
            path.display()
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Error;
    use regex_lite::Regex;

    fn directories(root: &Path) -> Result<ExtractDirectories> {
        Ok(ExtractDirectories::new(vec![(
            Regex::new("^postgresql$")?,
            root.to_path_buf(),
        )]))
    }

    #[test]
    fn test_empty_target_is_rejected() -> Result<()> {
        let root = tempfile::tempdir()?;
        let error = HardLink::new(
            root.path().join("UTC"),
            root.path().to_path_buf(),
            Path::new(""),
            &directories(root.path())?,
            true,
        )
        .expect_err("a hard link requires a nonempty target");
        assert!(error.to_string().contains("Missing hard link target"));
        assert!(!root.path().join("UTC").exists());
        Ok(())
    }

    #[test]
    fn test_destination_must_name_a_file() -> Result<()> {
        let root = tempfile::tempdir()?;
        fs::write(root.path().join("Universal"), b"timezone data")?;
        let link = HardLink::new(
            root.path().to_path_buf(),
            root.path().to_path_buf(),
            Path::new("postgresql/Universal"),
            &directories(root.path())?,
            true,
        )?;
        let error = link
            .extract()
            .expect_err("the extraction root cannot be replaced by a hard link");
        assert!(error.to_string().contains("Missing hard link file name"));
        assert!(root.path().is_dir());
        Ok(())
    }

    #[test]
    fn test_target_metadata_errors_are_propagated() -> Result<()> {
        let root = tempfile::tempdir()?;
        // Exceed the filename limit on the supported filesystems. Traversing a
        // regular file instead can report NotFound on Windows.
        let target = PathBuf::from("postgresql").join("x".repeat(300));
        let link = HardLink::new(
            root.path().join("UTC"),
            root.path().to_path_buf(),
            &target,
            &directories(root.path())?,
            true,
        )?;
        let error = link
            .extract()
            .expect_err("an invalid filename must not be treated as a missing target");
        assert!(matches!(error, Error::IoError(_)));
        assert!(!root.path().join("UTC").exists());
        Ok(())
    }

    #[test]
    fn test_destination_directory_errors_are_propagated() -> Result<()> {
        let root = tempfile::tempdir()?;
        fs::write(root.path().join("Universal"), b"timezone data")?;
        fs::write(root.path().join("file"), b"not a directory")?;
        let link = HardLink::new(
            root.path().join("file/Etc/UTC"),
            root.path().to_path_buf(),
            Path::new("postgresql/Universal"),
            &directories(root.path())?,
            true,
        )?;
        let error = link
            .extract()
            .expect_err("a file cannot be traversed as a destination directory");
        assert!(matches!(error, Error::IoError(_)));
        assert_eq!(fs::read(root.path().join("file"))?, b"not a directory");
        Ok(())
    }
}
