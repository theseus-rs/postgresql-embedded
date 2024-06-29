use crate::matcher::{default, postgresql_binaries};
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
    fn get<S: AsRef<str>>(&self, url: S) -> MatcherFn {
        let url = Some(url.as_ref().to_string());
        if let Some(matcher) = self.matchers.get(&url) {
            return *matcher.read().unwrap();
        }

        match self.matchers.get(&None) {
            Some(matcher) => *matcher.read().unwrap(),
            None => default::matcher,
        }
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
/// # Panics
/// * If the registry is poisoned.
#[allow(dead_code)]
pub fn register<S: AsRef<str>>(url: Option<S>, matcher_fn: MatcherFn) {
    let mut registry = REGISTRY.lock().unwrap();
    registry.register(url, matcher_fn);
}

/// Get a matcher for the specified URL, or the default matcher if no matcher is
/// registered for the URL.
///
/// # Panics
/// * If the registry is poisoned.
pub fn get<S: AsRef<str>>(url: S) -> MatcherFn {
    let registry = REGISTRY.lock().unwrap();
    registry.get(url)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_register() -> Result<()> {
        let matchers = REGISTRY.lock().unwrap().matchers.len();
        assert!(!REGISTRY.lock().unwrap().matchers.is_empty());
        REGISTRY.lock().unwrap().matchers.remove(&None::<String>);
        assert_ne!(matchers, REGISTRY.lock().unwrap().matchers.len());
        register(None::<&str>, default::matcher);
        assert_eq!(matchers, REGISTRY.lock().unwrap().matchers.len());

        let matcher = get(DEFAULT_POSTGRESQL_URL);
        let version = Version::new(16, 3, 0);
        let target = target_triple::TARGET;
        let name = format!("postgresql-{version}-{target}.tar.gz");

        assert!(matcher(name.as_str(), &version)?, "{}", name);
        Ok(())
    }

    #[test]
    fn test_default_matcher() -> Result<()> {
        let matcher = get("https://foo.com");
        let version = Version::new(16, 3, 0);
        let os = env::consts::OS;
        let arch = env::consts::ARCH;
        let name = format!("plugin_csv.pg16-{os}_{arch}.tar.gz");

        assert!(matcher(name.as_str(), &version)?, "{}", name);
        Ok(())
    }
}
