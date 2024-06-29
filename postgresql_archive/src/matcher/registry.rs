use crate::matcher::{default, postgresql_binaries};
use crate::Error::{PoisonedLock, UnsupportedMatcher};
use crate::{Result, DEFAULT_POSTGRESQL_URL};
use lazy_static::lazy_static;
use semver::Version;
use std::sync::{Arc, Mutex, RwLock};

lazy_static! {
    static ref REGISTRY: Arc<Mutex<MatchersRegistry>> =
        Arc::new(Mutex::new(MatchersRegistry::default()));
}

pub type SupportsFn = fn(&str) -> Result<bool>;
pub type MatcherFn = fn(&str, &Version) -> Result<bool>;

/// Singleton struct to store matchers
#[allow(clippy::type_complexity)]
struct MatchersRegistry {
    matchers: Vec<(Arc<RwLock<SupportsFn>>, Arc<RwLock<MatcherFn>>)>,
}

impl MatchersRegistry {
    /// Creates a new matcher registry.
    fn new() -> Self {
        Self {
            matchers: Vec::new(),
        }
    }

    /// Registers a matcher for a supports function. Newly registered matchers with the take
    /// precedence over existing ones.
    fn register(&mut self, supports_fn: SupportsFn, matcher_fn: MatcherFn) {
        self.matchers.insert(
            0,
            (
                Arc::new(RwLock::new(supports_fn)),
                Arc::new(RwLock::new(matcher_fn)),
            ),
        );
    }

    /// Get a matcher for the specified URL.
    ///
    /// # Errors
    /// * If the registry is poisoned.
    fn get<S: AsRef<str>>(&self, url: S) -> Result<MatcherFn> {
        let url = url.as_ref();
        for (supports_fn, matcher_fn) in &self.matchers {
            let supports_function = supports_fn
                .read()
                .map_err(|error| PoisonedLock(error.to_string()))?;
            if supports_function(url)? {
                let matcher_function = matcher_fn
                    .read()
                    .map_err(|error| PoisonedLock(error.to_string()))?;
                return Ok(*matcher_function);
            }
        }

        Err(UnsupportedMatcher(url.to_string()))
    }
}

impl Default for MatchersRegistry {
    /// Creates a new matcher registry with the default matchers registered.
    fn default() -> Self {
        let mut registry = Self::new();
        registry.register(
            |url| Ok(url == DEFAULT_POSTGRESQL_URL),
            postgresql_binaries::matcher,
        );
        registry.register(|_| Ok(true), default::matcher);
        registry
    }
}

/// Registers a matcher for a supports function. Newly registered matchers with the take
/// precedence over existing ones.
///
/// # Errors
/// * If the registry is poisoned.
#[allow(dead_code)]
pub fn register(supports_fn: SupportsFn, matcher_fn: MatcherFn) -> Result<()> {
    let mut registry = REGISTRY
        .lock()
        .map_err(|error| PoisonedLock(error.to_string()))?;
    registry.register(supports_fn, matcher_fn);
    Ok(())
}

/// Get a matcher for the specified URL.
///
/// # Errors
/// * If the registry is poisoned.
pub fn get<S: AsRef<str>>(url: S) -> Result<MatcherFn> {
    let registry = REGISTRY
        .lock()
        .map_err(|error| PoisonedLock(error.to_string()))?;
    registry.get(url)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_register() -> Result<()> {
        register(
            |url| Ok(url == "https://foo.com"),
            |name, _| Ok(name == "foo"),
        )?;

        let matcher = get("https://foo.com")?;
        let version = Version::new(16, 3, 0);

        assert!(matcher("foo", &version)?);
        Ok(())
    }

    #[test]
    fn test_default_matcher() -> Result<()> {
        let matcher = get("https://foo.com")?;
        let version = Version::new(16, 3, 0);
        let os = env::consts::OS;
        let arch = env::consts::ARCH;
        let name = format!("plugin_csv.pg16-{os}_{arch}.tar.gz");

        assert!(matcher(name.as_str(), &version)?, "{}", name);
        Ok(())
    }
}
