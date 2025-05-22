use crate::Error::{PoisonedLock, UnsupportedMatcher};
use crate::Result;
use crate::configuration::matcher;
#[cfg(feature = "theseus")]
use crate::configuration::theseus;
#[cfg(feature = "zonky")]
use crate::configuration::zonky;
use semver::Version;
use std::sync::{Arc, LazyLock, Mutex, RwLock};

static REGISTRY: LazyLock<Arc<Mutex<MatchersRegistry>>> =
    LazyLock::new(|| Arc::new(Mutex::new(MatchersRegistry::default())));

pub type SupportsFn = fn(&str) -> Result<bool>;
pub type MatcherFn = fn(&str, &str, &Version) -> Result<bool>;

/// Singleton struct to store matchers
#[expect(clippy::type_complexity)]
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
        #[cfg(feature = "theseus")]
        registry.register(|url| Ok(url == theseus::URL), matcher);
        #[cfg(feature = "zonky")]
        registry.register(|url| Ok(url == zonky::URL), zonky::matcher);
        registry
    }
}

/// Registers a matcher for a supports function. Newly registered matchers with the take
/// precedence over existing ones.
///
/// # Errors
/// * If the registry is poisoned.
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

    #[test]
    fn test_register() -> Result<()> {
        register(
            |url| Ok(url == "https://foo.com"),
            |_url, name, _version| Ok(name == "foo"),
        )?;

        let matcher = get("https://foo.com")?;
        let version = Version::new(16, 3, 0);

        assert!(matcher("", "foo", &version)?);
        Ok(())
    }

    #[test]
    fn test_get_error() {
        let result = get("foo").unwrap_err();
        assert_eq!("unsupported matcher for 'foo'", result.to_string());
    }

    #[test]
    #[cfg(feature = "theseus")]
    fn test_get_theseus_postgresql_binaries() {
        assert!(get(theseus::URL).is_ok());
    }

    #[test]
    #[cfg(feature = "zonky")]
    fn test_get_zonyk_postgresql_binaries() {
        assert!(get(zonky::URL).is_ok());
    }
}
