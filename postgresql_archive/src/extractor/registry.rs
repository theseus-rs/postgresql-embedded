#[cfg(feature = "theseus")]
use crate::configuration::theseus;
#[cfg(feature = "zonky")]
use crate::configuration::zonky;
use crate::extractor::ExtractDirectories;
use crate::Error::{PoisonedLock, UnsupportedExtractor};
use crate::Result;
use std::path::PathBuf;
use std::sync::{Arc, LazyLock, Mutex, RwLock};

static REGISTRY: LazyLock<Arc<Mutex<RepositoryRegistry>>> =
    LazyLock::new(|| Arc::new(Mutex::new(RepositoryRegistry::default())));

type SupportsFn = fn(&str) -> Result<bool>;
type ExtractFn = fn(&Vec<u8>, ExtractDirectories) -> Result<Vec<PathBuf>>;

/// Singleton struct to store extractors
#[expect(clippy::type_complexity)]
struct RepositoryRegistry {
    extractors: Vec<(Arc<RwLock<SupportsFn>>, Arc<RwLock<ExtractFn>>)>,
}

impl RepositoryRegistry {
    /// Creates a new extractor registry.
    fn new() -> Self {
        Self {
            extractors: Vec::new(),
        }
    }

    /// Registers an extractor. Newly registered extractors take precedence over existing ones.
    fn register(&mut self, supports_fn: SupportsFn, extract_fn: ExtractFn) {
        self.extractors.insert(
            0,
            (
                Arc::new(RwLock::new(supports_fn)),
                Arc::new(RwLock::new(extract_fn)),
            ),
        );
    }

    /// Gets an extractor that supports the specified URL
    ///
    /// # Errors
    /// * If the URL is not supported.
    fn get(&self, url: &str) -> Result<ExtractFn> {
        for (supports_fn, extractor_fn) in &self.extractors {
            let supports_function = supports_fn
                .read()
                .map_err(|error| PoisonedLock(error.to_string()))?;
            if supports_function(url)? {
                let extractor_function = extractor_fn
                    .read()
                    .map_err(|error| PoisonedLock(error.to_string()))?;
                return Ok(*extractor_function);
            }
        }

        Err(UnsupportedExtractor(url.to_string()))
    }
}

impl Default for RepositoryRegistry {
    /// Creates a new repository registry with the default repositories registered.
    fn default() -> Self {
        let mut registry = Self::new();
        #[cfg(feature = "theseus")]
        registry.register(|url| Ok(url.starts_with(theseus::URL)), theseus::extract);
        #[cfg(feature = "zonky")]
        registry.register(|url| Ok(url.starts_with(zonky::URL)), zonky::extract);
        registry
    }
}

/// Registers an extractor. Newly registered extractors take precedence over existing ones.
///
/// # Errors
/// * If the registry is poisoned.
pub fn register(supports_fn: SupportsFn, extractor_fn: ExtractFn) -> Result<()> {
    let mut registry = REGISTRY
        .lock()
        .map_err(|error| PoisonedLock(error.to_string()))?;
    registry.register(supports_fn, extractor_fn);
    Ok(())
}

/// Gets an extractor that supports the specified URL
///
/// # Errors
/// * If the URL is not supported.
pub fn get(url: &str) -> Result<ExtractFn> {
    let registry = REGISTRY
        .lock()
        .map_err(|error| PoisonedLock(error.to_string()))?;
    registry.get(url)
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;

    #[test]
    fn test_register() -> Result<()> {
        register(|url| Ok(url == "https://foo.com"), |_, _| Ok(Vec::new()))?;
        let url = "https://foo.com";
        let extractor = get(url)?;
        let mut extract_directories = ExtractDirectories::default();
        extract_directories.add_mapping(Regex::new(".*")?, PathBuf::from("test"));
        assert!(extractor(&Vec::new(), extract_directories).is_ok());
        Ok(())
    }

    #[test]
    fn test_get_error() {
        let error = get("foo").unwrap_err();
        assert_eq!("unsupported extractor for 'foo'", error.to_string());
    }

    #[test]
    #[cfg(feature = "theseus")]
    fn test_get_theseus_postgresql_binaries() {
        assert!(get(theseus::URL).is_ok());
    }
}
