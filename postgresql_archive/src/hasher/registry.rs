use crate::hasher::{blake2b_512, blake2s_256, sha2_256, sha2_512, sha3_256, sha3_512};
use crate::Error::{PoisonedLock, UnsupportedRepository};
use crate::Result;
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

        Err(UnsupportedRepository(url.to_string()))
    }
}

impl Default for HasherRegistry {
    /// Creates a new hasher registry with the default hashers registered.
    fn default() -> Self {
        let mut registry = Self::new();
        registry.register(|_, extension| Ok(extension == "blake2s"), blake2s_256::hash);
        registry.register(|_, extension| Ok(extension == "blake2b"), blake2b_512::hash);
        registry.register(|_, extension| Ok(extension == "sha256"), sha2_256::hash);
        registry.register(|_, extension| Ok(extension == "sha512"), sha2_512::hash);
        registry.register(|_, extension| Ok(extension == "sha3-256"), sha3_256::hash);
        registry.register(|_, extension| Ok(extension == "sha3-512"), sha3_512::hash);
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
            |_, extension| Ok(extension == "foo"),
            |_| Ok("42".to_string()),
        )?;
        test_hasher("foo", "42")
    }

    #[test]
    fn test_sha2_256() -> Result<()> {
        test_hasher(
            "sha256",
            "9a89c68c4c5e28b8c4a5567673d462fff515db46116f9900624d09c474f593fb",
        )
    }

    #[test]
    fn test_sha2_512() -> Result<()> {
        test_hasher(
            "sha512",
            "3ad3f36979450d4f53366244ecf1010f4f9121d6888285ff14104fd5aded85d48aa171bf1e33a112602f92b7a7088b298789012fb87b9056321241a19fb74e0b",
        )
    }

    #[test]
    fn test_sha3_256() -> Result<()> {
        test_hasher(
            "sha3-256",
            "c0188232190e0427fc9cc78597221c76c799528660889bd6ce1f3563148ff84d",
        )
    }

    #[test]
    fn test_sha3_512() -> Result<()> {
        test_hasher(
            "sha3-512",
            "9429fc1f9772cc1d8039fe75cc1b033cd60f0ec4face0f8a514d25b0649ba8a5954b6c7a41cc3697a56db3ff321475be1fa14b70c7eb78fec6ce62dbfc54c9d3",
        )
    }

    #[test]
    fn test_blake2s_256() -> Result<()> {
        test_hasher(
            "blake2s",
            "7125921e06071710350390fe902856dbea366a5d6f5ee26c18e741143ac80061",
        )
    }

    #[test]
    fn test_blake2b_512() -> Result<()> {
        test_hasher(
            "blake2b",
            "67767f1cab415502dcceec9f099fb84539b1c73c5ebdcfe1bb8ca7411e3b6cb33e304f49222edac9bdaa74129e9e13f11f215b8560f9081f0e8f1f869162bf46",
        )
    }
}
