#[cfg(feature = "theseus")]
use crate::configuration::theseus;
#[cfg(feature = "md5")]
use crate::hasher::md5;
#[cfg(feature = "sha1")]
use crate::hasher::sha1;
#[cfg(feature = "sha2")]
use crate::hasher::sha2_256;
#[cfg(feature = "sha2")]
use crate::hasher::sha2_512;
#[cfg(feature = "maven")]
use crate::repository::maven;
use crate::Error::{PoisonedLock, UnsupportedHasher};
use crate::Result;
use std::sync::{Arc, LazyLock, Mutex, RwLock};

static REGISTRY: LazyLock<Arc<Mutex<HasherRegistry>>> =
    LazyLock::new(|| Arc::new(Mutex::new(HasherRegistry::default())));

pub type SupportsFn = fn(&str, &str) -> Result<bool>;
pub type HasherFn = fn(&Vec<u8>) -> Result<String>;

/// Singleton struct to store hashers
#[expect(clippy::type_complexity)]
struct HasherRegistry {
    hashers: Vec<(Arc<RwLock<SupportsFn>>, Arc<RwLock<HasherFn>>)>,
}

impl HasherRegistry {
    /// Creates a new hasher registry.
    fn new() -> Self {
        Self {
            hashers: Vec::new(),
        }
    }

    /// Registers a hasher for a supports function. Newly registered hashers will take precedence
    /// over existing ones.
    fn register(&mut self, supports_fn: SupportsFn, hasher_fn: HasherFn) {
        self.hashers.insert(
            0,
            (
                Arc::new(RwLock::new(supports_fn)),
                Arc::new(RwLock::new(hasher_fn)),
            ),
        );
    }

    /// Get a hasher for the specified url and extension.
    ///
    /// # Errors
    /// * If the registry is poisoned.
    fn get<S: AsRef<str>>(&self, url: S, extension: S) -> Result<HasherFn> {
        let url = url.as_ref();
        let extension = extension.as_ref();
        for (supports_fn, hasher_fn) in &self.hashers {
            let supports_function = supports_fn
                .read()
                .map_err(|error| PoisonedLock(error.to_string()))?;
            if supports_function(url, extension)? {
                let hasher_function = hasher_fn
                    .read()
                    .map_err(|error| PoisonedLock(error.to_string()))?;
                return Ok(*hasher_function);
            }
        }

        Err(UnsupportedHasher(url.to_string()))
    }
}

impl Default for HasherRegistry {
    /// Creates a new hasher registry with the default hashers registered.
    fn default() -> Self {
        let mut registry = Self::new();
        #[cfg(feature = "theseus")]
        registry.register(
            |url, extension| Ok(url.starts_with(theseus::URL) && extension == "sha256"),
            sha2_256::hash,
        );
        // Register the Maven hashers: https://maven.apache.org/resolver/about-checksums.html#implemented-checksum-algorithms
        #[cfg(feature = "maven")]
        registry.register(
            |url, extension| Ok(url.starts_with(maven::URL) && extension == "md5"),
            md5::hash,
        );
        #[cfg(feature = "maven")]
        registry.register(
            |url, extension| Ok(url.starts_with(maven::URL) && extension == "sha1"),
            sha1::hash,
        );
        #[cfg(feature = "maven")]
        registry.register(
            |url, extension| Ok(url.starts_with(maven::URL) && extension == "sha256"),
            sha2_256::hash,
        );
        #[cfg(feature = "maven")]
        registry.register(
            |url, extension| Ok(url.starts_with(maven::URL) && extension == "sha512"),
            sha2_512::hash,
        );
        registry
    }
}

/// Registers a hasher for a supports function. Newly registered hashers will take precedence
/// over existing ones.
///
/// # Errors
/// * If the registry is poisoned.
pub fn register(supports_fn: SupportsFn, hasher_fn: HasherFn) -> Result<()> {
    let mut registry = REGISTRY
        .lock()
        .map_err(|error| PoisonedLock(error.to_string()))?;
    registry.register(supports_fn, hasher_fn);
    Ok(())
}

/// Get a hasher for the specified url and extension.
///
/// # Errors
/// * If the registry is poisoned.
pub fn get<S: AsRef<str>>(url: S, extension: S) -> Result<HasherFn> {
    let registry = REGISTRY
        .lock()
        .map_err(|error| PoisonedLock(error.to_string()))?;
    registry.get(url, extension)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_hasher(extension: &str, expected: &str) -> Result<()> {
        let hasher = get("https://foo.com", extension)?;
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
        let hash = hasher(&data)?;
        assert_eq!(expected, hash);
        Ok(())
    }

    #[test]
    fn test_register() -> Result<()> {
        register(
            |_, extension| Ok(extension == "test"),
            |_| Ok("42".to_string()),
        )?;
        test_hasher("test", "42")
    }

    #[test]
    fn test_get_invalid_url_error() {
        let error = get("https://foo.com", "foo").unwrap_err();
        assert_eq!(
            "unsupported hasher for 'https://foo.com'",
            error.to_string()
        );
    }

    #[test]
    #[cfg(feature = "theseus")]
    fn test_get_invalid_extension_error() {
        let error = get(theseus::URL, "foo").unwrap_err();
        assert_eq!(
            format!("unsupported hasher for '{}'", theseus::URL),
            error.to_string()
        );
    }

    #[test]
    #[cfg(feature = "theseus")]
    fn test_get_theseus_postgresql_binaries() {
        assert!(get(theseus::URL, "sha256").is_ok());
    }

    #[test]
    #[cfg(feature = "maven")]
    fn test_get_zonky_postgresql_binaries() {
        assert!(get(maven::URL, "sha512").is_ok());
    }
}
