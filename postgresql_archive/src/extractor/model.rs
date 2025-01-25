use crate::{Error, Result};
use regex_lite::Regex;
use std::fmt::Display;
use std::path::PathBuf;

/// Extract directories manage the directories to extract a file in an archive to based upon the
/// associated regex matching the file path.
#[derive(Debug)]
pub struct ExtractDirectories {
    mappings: Vec<(Regex, PathBuf)>,
}

impl ExtractDirectories {
    /// Creates a new ExtractDirectories instance.
    #[must_use]
    pub fn new(mappings: Vec<(Regex, PathBuf)>) -> Self {
        Self { mappings }
    }

    /// Adds a new mapping to the ExtractDirectories instance.
    pub fn add_mapping(&mut self, regex: Regex, path: PathBuf) {
        self.mappings.push((regex, path));
    }

    /// Returns the path associated with the first regex that matches the file path.
    /// If no regex matches, then the file path is returned.
    ///
    /// # Errors
    /// Returns an error if the file path cannot be converted to a string.
    pub fn get_path(&self, file_path: &str) -> Result<PathBuf> {
        for (regex, path) in &self.mappings {
            if regex.is_match(file_path) {
                return Ok(path.clone());
            }
        }
        Err(Error::Unexpected(format!(
            "No regex matched the file path: {file_path}"
        )))
    }
}

/// Default implementation for ExtractDirectories.
impl Default for ExtractDirectories {
    /// Creates a new ExtractDirectories instance with an empty mappings vector.
    fn default() -> Self {
        ExtractDirectories::new(Vec::new())
    }
}

/// Display implementation for ExtractDirectories.
impl Display for ExtractDirectories {
    /// Formats the ExtractDirectories instance.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (regex, path) in &self.mappings {
            writeln!(f, "{} -> {}", regex, path.display())?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() -> Result<()> {
        let mappings = vec![(Regex::new(".*")?, PathBuf::from("test"))];
        let extract_directories = ExtractDirectories::new(mappings);
        let path = extract_directories.get_path("foo")?;
        assert_eq!("test", path.to_string_lossy());
        Ok(())
    }

    #[test]
    fn test_default() {
        let extract_directories = ExtractDirectories::default();
        let result = extract_directories.get_path("foo");
        assert!(result.is_err());
    }

    #[test]
    fn test_add_mapping() -> Result<()> {
        let mut extract_directories = ExtractDirectories::default();
        extract_directories.add_mapping(Regex::new(".*")?, PathBuf::from("test"));
        let path = extract_directories.get_path("foo")?;
        assert_eq!("test", path.to_string_lossy());
        Ok(())
    }

    #[test]
    fn test_get_path() -> Result<()> {
        let mappings = vec![
            (Regex::new("test")?, PathBuf::from("test")),
            (Regex::new("foo")?, PathBuf::from("bar")),
        ];
        let extract_directories = ExtractDirectories::new(mappings);
        let path = extract_directories.get_path("foo")?;
        assert_eq!("bar", path.to_string_lossy());
        Ok(())
    }

    #[test]
    fn test_display() -> Result<()> {
        let mappings = vec![
            (Regex::new("test")?, PathBuf::from("test")),
            (Regex::new("foo")?, PathBuf::from("bar")),
        ];
        let extract_directories = ExtractDirectories::new(mappings);
        let display = extract_directories.to_string();
        assert_eq!("test -> test\nfoo -> bar\n", display);
        Ok(())
    }
}
