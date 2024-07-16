#[cfg(feature = "theseus")]
use crate::configuration::theseus;
#[cfg(feature = "zonky")]
use crate::configuration::zonky;
use crate::Error::{PoisonedLock, UnsupportedExtractor};
use crate::Result;
use lazy_static::lazy_static;
use std::path::Path;
use std::sync::{Arc, Mutex, RwLock};

lazy_static! {
    static ref REGISTRY: Arc<Mutex<RepositoryRegistry>> =
        Arc::new(Mutex::new(RepositoryRegistry::default()));
}

type SupportsFn = fn(&str) -> Result<bool>;
type ExtractFn = fn(&Vec<u8>, &Path) -> Result<()>;

/// Singleton struct to store extractors
#[allow(clippy::type_complexity)]
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
        #[allow(unused_mut)]
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
#[allow(dead_code)]
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

    #[test]
    fn test_register() -> Result<()> {
        register(|url| Ok(url == "https://foo.com"), |_, _| Ok(()))?;
        let url = "https://foo.com";
        let extractor = get(url)?;
        assert!(extractor(&Vec::new(), Path::new("foo")).is_ok());
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
