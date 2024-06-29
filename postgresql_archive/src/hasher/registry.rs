use crate::hasher::{blake2b_512, blake2s_256, sha2_256, sha2_512, sha3_256, sha3_512};
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
    fn new() -> Self {
        Self {
            hashers: HashMap::new(),
        }
    }

    /// Registers a hasher for an extension. Newly registered hashers with the same extension will
    /// override existing ones.
    fn register<S: AsRef<str>>(&mut self, extension: S, hasher_fn: HasherFn) {
        let extension = extension.as_ref().to_string();
        self.hashers
            .insert(extension, Arc::new(RwLock::new(hasher_fn)));
    }

    /// Get a hasher for the specified extension.
    fn get<S: AsRef<str>>(&self, extension: S) -> Option<HasherFn> {
        let extension = extension.as_ref().to_string();
        if let Some(hasher) = self.hashers.get(&extension) {
            return Some(*hasher.read().unwrap());
        }

        None
    }
}

impl Default for HasherRegistry {
    /// Creates a new hasher registry with the default hashers registered.
    fn default() -> Self {
        let mut registry = Self::new();
        registry.register("blake2s", blake2s_256::hash);
        registry.register("blake2b", blake2b_512::hash);
        registry.register("sha256", sha2_256::hash);
        registry.register("sha512", sha2_512::hash);
        registry.register("sha3-256", sha3_256::hash);
        registry.register("sha3-512", sha3_512::hash);
        registry
    }
}

/// Registers a hasher for an extension. Newly registered hashers with the same extension will
/// override existing ones.
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

    #[test]
    fn test_sha2_512() -> Result<()> {
        let hasher = get("sha512").unwrap();
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
        let hash = hasher(&data)?;

        assert_eq!(
            "3ad3f36979450d4f53366244ecf1010f4f9121d6888285ff14104fd5aded85d48aa171bf1e33a112602f92b7a7088b298789012fb87b9056321241a19fb74e0b",
            hash
        );
        Ok(())
    }

    #[test]
    fn test_sha3_256() -> Result<()> {
        let hasher = get("sha3-256").unwrap();
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
        let hash = hasher(&data)?;

        assert_eq!(
            "c0188232190e0427fc9cc78597221c76c799528660889bd6ce1f3563148ff84d",
            hash
        );
        Ok(())
    }

    #[test]
    fn test_sha3_512() -> Result<()> {
        let hasher = get("sha3-512").unwrap();
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
        let hash = hasher(&data)?;

        assert_eq!(
            "9429fc1f9772cc1d8039fe75cc1b033cd60f0ec4face0f8a514d25b0649ba8a5954b6c7a41cc3697a56db3ff321475be1fa14b70c7eb78fec6ce62dbfc54c9d3",
            hash
        );
        Ok(())
    }

    #[test]
    fn test_blake2s_256() -> Result<()> {
        let hasher = get("blake2s").unwrap();
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
        let hash = hasher(&data)?;

        assert_eq!(
            "7125921e06071710350390fe902856dbea366a5d6f5ee26c18e741143ac80061",
            hash
        );
        Ok(())
    }

    #[test]
    fn test_blake2s_512() -> Result<()> {
        let hasher = get("blake2s").unwrap();
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
        let hash = hasher(&data)?;

        assert_eq!(
            "7125921e06071710350390fe902856dbea366a5d6f5ee26c18e741143ac80061",
            hash
        );
        Ok(())
    }
}
