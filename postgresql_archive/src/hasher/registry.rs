use crate::hasher::sha2_256;
use crate::Error::{PoisonedLock, UnsupportedHasher};
use crate::{Result, THESEUS_POSTGRESQL_BINARIES_URL};
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex, RwLock};

lazy_static! {
    static ref REGISTRY: Arc<Mutex<HasherRegistry>> =
        Arc::new(Mutex::new(HasherRegistry::default()));
}

pub type SupportsFn = fn(&str, &str) -> Result<bool>;
pub type HasherFn = fn(&Vec<u8>) -> Result<String>;

/// Singleton struct to store hashers
#[allow(clippy::type_complexity)]
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
        registry.register(
            |url, extension| {
                Ok(url.starts_with(THESEUS_POSTGRESQL_BINARIES_URL) && extension == "sha256")
            },
            sha2_256::hash,
        );
        registry
    }
}

/// Registers a hasher for a supports function. Newly registered hashers will take precedence
/// over existing ones.
///
/// # Errors
/// * If the registry is poisoned.
#[allow(dead_code)]
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
    fn test_get_invalid_extension_error() {
        let error = get(THESEUS_POSTGRESQL_BINARIES_URL, "foo").unwrap_err();
        assert_eq!(
            format!("unsupported hasher for '{THESEUS_POSTGRESQL_BINARIES_URL}'"),
            error.to_string()
        );
    }

    #[test]
    fn test_get_theseus_postgresql_binaries() {
        assert!(get(THESEUS_POSTGRESQL_BINARIES_URL, "sha256").is_ok());
    }
}
