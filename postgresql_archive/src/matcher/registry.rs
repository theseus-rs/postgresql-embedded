use crate::matcher::{default, postgresql_binaries};
use crate::Error::PoisonedLock;
use crate::{Result, DEFAULT_POSTGRESQL_URL};
use lazy_static::lazy_static;
use semver::Version;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};

lazy_static! {
    static ref REGISTRY: Arc<Mutex<MatchersRegistry>> =
        Arc::new(Mutex::new(MatchersRegistry::default()));
}

pub type MatcherFn = fn(&str, &Version) -> Result<bool>;

/// Singleton struct to store matchers
struct MatchersRegistry {
    matchers: HashMap<Option<String>, Arc<RwLock<MatcherFn>>>,
}

impl MatchersRegistry {
    /// Creates a new matcher registry.
    fn new() -> Self {
        Self {
            matchers: HashMap::new(),
        }
    }

    /// Registers a matcher for a URL. Newly registered matchers with the same url will override
    /// existing ones.
    fn register<S: AsRef<str>>(&mut self, url: Option<S>, matcher_fn: MatcherFn) {
        let url: Option<String> = url.map(|s| s.as_ref().to_string());
        self.matchers.insert(url, Arc::new(RwLock::new(matcher_fn)));
    }

    /// Get a matcher for the specified URL, or the default matcher if no matcher is
    /// registered for the URL.
    ///
    /// # Errors
    /// * If the registry is poisoned.
    fn get<S: AsRef<str>>(&self, url: S) -> Result<MatcherFn> {
        let url = Some(url.as_ref().to_string());
        if let Some(matcher) = self.matchers.get(&url) {
            let matcher = *matcher
                .read()
                .map_err(|error| PoisonedLock(error.to_string()))?;
            return Ok(matcher);
        }

        let matcher = match self.matchers.get(&None) {
            Some(matcher) => *matcher
                .read()
                .map_err(|error| PoisonedLock(error.to_string()))?,
            None => default::matcher,
        };
        Ok(matcher)
    }

    /// Get the number of matchers in the registry.
    fn len(&self) -> usize {
        self.matchers.len()
    }

    /// Check if the registry is empty.
    fn is_empty(&self) -> bool {
        self.matchers.is_empty()
    }
}

impl Default for MatchersRegistry {
    /// Creates a new matcher registry with the default matchers registered.
    fn default() -> Self {
        let mut registry = Self::new();
        registry.register(None::<&str>, default::matcher);
        registry.register(Some(DEFAULT_POSTGRESQL_URL), postgresql_binaries::matcher);
        registry
    }
}

/// Registers a matcher for a URL. Newly registered matchers with the same url will override
/// existing ones.
///
/// # Errors
/// * If the registry is poisoned.
#[allow(dead_code)]
pub fn register<S: AsRef<str>>(url: Option<S>, matcher_fn: MatcherFn) -> Result<()> {
    let mut registry = REGISTRY
        .lock()
        .map_err(|error| PoisonedLock(error.to_string()))?;
    registry.register(url, matcher_fn);
    Ok(())
}

/// Get a matcher for the specified URL, or the default matcher if no matcher is
/// registered for the URL.
///
/// # Errors
/// * If the registry is poisoned.
pub fn get<S: AsRef<str>>(url: S) -> Result<MatcherFn> {
    let registry = REGISTRY
        .lock()
        .map_err(|error| PoisonedLock(error.to_string()))?;
    registry.get(url)
}

/// Get the number of matchers in the registry.
///
/// # Errors
/// * If the registry is poisoned.
pub fn len() -> Result<usize> {
    let registry = REGISTRY
        .lock()
        .map_err(|error| PoisonedLock(error.to_string()))?;
    Ok(registry.len())
}

/// Check if the registry is empty.
///
/// # Errors
/// * If the registry is poisoned.
pub fn is_empty() -> Result<bool> {
    let registry = REGISTRY
        .lock()
        .map_err(|error| PoisonedLock(error.to_string()))?;
    Ok(registry.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Error::PoisonedLock;
    use std::env;

    #[test]
    fn test_register() -> Result<()> {
        let matchers = len()?;
        assert!(!is_empty()?);
        REGISTRY
            .lock()
            .map_err(|error| PoisonedLock(error.to_string()))?
            .matchers
            .remove(&None::<String>);
        assert_ne!(matchers, len()?);
        register(None::<&str>, default::matcher)?;
        assert_eq!(matchers, len()?);

        let matcher = get(DEFAULT_POSTGRESQL_URL)?;
        let version = Version::new(16, 3, 0);
        let target = target_triple::TARGET;
        let name = format!("postgresql-{version}-{target}.tar.gz");

        assert!(matcher(name.as_str(), &version)?, "{}", name);
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
