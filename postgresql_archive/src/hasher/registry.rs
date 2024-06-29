use crate::hasher::sha2_256;
use crate::Result;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};

lazy_static! {
    static ref REGISTRY: Arc<Mutex<HasherRegistry>> =
        Arc::new(Mutex::new(HasherRegistry::default()));
}

pub type HasherFn = fn(&Vec<u8>) -> Result<String>;

/// Singleton struct to store hashers
struct HasherRegistry {
    hashers: HashMap<String, Arc<RwLock<HasherFn>>>,
}

impl HasherRegistry {
    /// Creates a new hasher registry.
    ///
    /// # Returns
    /// * The hasher registry.
    fn new() -> Self {
        Self {
            hashers: HashMap::new(),
        }
    }

    /// Registers a hasher for an extension. Newly registered hashers with the same extension will
    /// override existing ones.
    ///
    /// # Arguments
    /// * `extension` - The extension to register the hasher for.
    /// * `hasher_fn` - The hasher function to register.
    fn register<S: AsRef<str>>(&mut self, extension: S, hasher_fn: HasherFn) {
        let extension = extension.as_ref().to_string();
        self.hashers
            .insert(extension, Arc::new(RwLock::new(hasher_fn)));
    }

    /// Get a hasher for the specified extension.
    ///
    /// # Arguments
    /// * `extension` - The extension to locate a hasher for.
    ///
    /// # Returns
    /// * The hasher for the extension or [None] if not found.
    fn get<S: AsRef<str>>(&self, extension: S) -> Option<HasherFn> {
        let extension = extension.as_ref().to_string();
        if let Some(hasher) = self.hashers.get(&extension) {
            return Some(*hasher.read().unwrap());
        }

        None
    }
}

impl Default for HasherRegistry {
    fn default() -> Self {
        let mut registry = Self::new();
        registry.register("sha256", sha2_256::hash);
        registry
    }
}

/// Registers a hasher for an extension. Newly registered hashers with the same extension will
/// override existing ones.
///
/// # Arguments
/// * `extension` - The extension to register the hasher for.
/// * `hasher_fn` - The hasher function to register.
///
/// # Panics
/// * If the registry is poisoned.
#[allow(dead_code)]
pub fn register<S: AsRef<str>>(extension: S, hasher_fn: HasherFn) {
    let mut registry = REGISTRY.lock().unwrap();
    registry.register(extension, hasher_fn);
}

/// Get a hasher for the specified extension.
///
/// # Arguments
/// * `extension` - The extension to locate a hasher for.
///
/// # Returns
/// * The hasher for the extension or [None] if not found.
///
/// # Panics
/// * If the registry is poisoned.
pub fn get<S: AsRef<str>>(extension: S) -> Option<HasherFn> {
    let registry = REGISTRY.lock().unwrap();
    registry.get(extension)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() -> Result<()> {
        let extension = "sha256";
        let hashers = REGISTRY.lock().unwrap().hashers.len();
        assert!(!REGISTRY.lock().unwrap().hashers.is_empty());
        REGISTRY.lock().unwrap().hashers.remove(extension);
        assert_ne!(hashers, REGISTRY.lock().unwrap().hashers.len());
        register(extension, sha2_256::hash);
        assert_eq!(hashers, REGISTRY.lock().unwrap().hashers.len());

        let hasher = get(extension).unwrap();
        let data = vec![1, 2, 3];
        let hash = hasher(&data)?;

        assert_eq!(
            "039058c6f2c0cb492c533b0a4d14ef77cc0f78abccced5287d84a1a2011cfb81",
            hash
        );
        Ok(())
    }

    #[test]
    fn test_sha2_256() -> Result<()> {
        let hasher = get("sha256").unwrap();
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
        let hash = hasher(&data)?;

        assert_eq!(
            "9a89c68c4c5e28b8c4a5567673d462fff515db46116f9900624d09c474f593fb",
            hash
        );
        Ok(())
    }
}
