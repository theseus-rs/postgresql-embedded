#[cfg(feature = "theseus")]
use crate::configuration::theseus;
#[cfg(feature = "zonky")]
use crate::configuration::zonky;
#[cfg(feature = "github")]
use crate::repository::github::repository::GitHub;
use crate::repository::model::Repository;
use crate::Error::{PoisonedLock, UnsupportedRepository};
use crate::Result;
use std::sync::{Arc, LazyLock, Mutex, RwLock};

static REGISTRY: LazyLock<Arc<Mutex<RepositoryRegistry>>> =
    LazyLock::new(|| Arc::new(Mutex::new(RepositoryRegistry::default())));

type SupportsFn = fn(&str) -> Result<bool>;
type NewFn = dyn Fn(&str) -> Result<Box<dyn Repository>> + Send + Sync;

/// Singleton struct to store repositories
#[expect(clippy::type_complexity)]
struct RepositoryRegistry {
    repositories: Vec<(Arc<RwLock<SupportsFn>>, Arc<RwLock<NewFn>>)>,
}

impl RepositoryRegistry {
    /// Creates a new repository registry.
    fn new() -> Self {
        Self {
            repositories: Vec::new(),
        }
    }

    /// Registers a repository. Newly registered repositories take precedence over existing ones.
    fn register(&mut self, supports_fn: SupportsFn, new_fn: Box<NewFn>) {
        self.repositories.insert(
            0,
            (
                Arc::new(RwLock::new(supports_fn)),
                Arc::new(RwLock::new(new_fn)),
            ),
        );
    }

    /// Gets a repository that supports the specified URL
    ///
    /// # Errors
    /// * If the URL is not supported.
    fn get(&self, url: &str) -> Result<Box<dyn Repository>> {
        for (supports_fn, new_fn) in &self.repositories {
            let supports_function = supports_fn
                .read()
                .map_err(|error| PoisonedLock(error.to_string()))?;
            if supports_function(url)? {
                let new_function = new_fn
                    .read()
                    .map_err(|error| PoisonedLock(error.to_string()))?;
                return new_function(url);
            }
        }

        Err(UnsupportedRepository(url.to_string()))
    }
}

impl Default for RepositoryRegistry {
    /// Creates a new repository registry with the default repositories registered.
    fn default() -> Self {
        let mut registry = Self::new();
        #[cfg(feature = "theseus")]
        registry.register(
            |url| Ok(url.starts_with(theseus::URL)),
            Box::new(GitHub::new),
        );
        #[cfg(feature = "zonky")]
        registry.register(
            |url| Ok(url.starts_with(zonky::URL)),
            Box::new(zonky::Zonky::new),
        );
        registry
    }
}

/// Registers a repository. Newly registered repositories can override existing ones.
///
/// # Errors
/// * If the registry is poisoned.
pub fn register(supports_fn: SupportsFn, new_fn: Box<NewFn>) -> Result<()> {
    let mut registry = REGISTRY
        .lock()
        .map_err(|error| PoisonedLock(error.to_string()))?;
    registry.register(supports_fn, new_fn);
    Ok(())
}

/// Gets a repository that supports the specified URL
///
/// # Errors
/// * If the URL is not supported.
pub fn get(url: &str) -> Result<Box<dyn Repository>> {
    let registry = REGISTRY
        .lock()
        .map_err(|error| PoisonedLock(error.to_string()))?;
    registry.get(url)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository::Archive;
    use async_trait::async_trait;
    use semver::{Version, VersionReq};
    use std::fmt::Debug;

    #[derive(Debug)]
    struct TestRepository;

    impl TestRepository {
        #[expect(clippy::new_ret_no_self)]
        #[expect(clippy::unnecessary_wraps)]
        fn new(_url: &str) -> Result<Box<dyn Repository>> {
            Ok(Box::new(Self))
        }
    }

    #[async_trait]
    impl Repository for TestRepository {
        fn name(&self) -> &str {
            "test"
        }

        async fn get_version(&self, _version_req: &VersionReq) -> Result<Version> {
            Ok(Version::new(0, 0, 42))
        }

        async fn get_archive(&self, _version_req: &VersionReq) -> Result<Archive> {
            Ok(Archive::new(
                "test".to_string(),
                Version::new(0, 0, 42),
                Vec::new(),
            ))
        }
    }

    #[tokio::test]
    async fn test_register() -> Result<()> {
        register(
            |url| Ok(url == "https://foo.com"),
            Box::new(TestRepository::new),
        )?;
        let url = "https://foo.com";
        let repository = get(url)?;
        assert_eq!("test", repository.name());
        assert!(repository.get_version(&VersionReq::STAR).await.is_ok());
        assert!(repository.get_archive(&VersionReq::STAR).await.is_ok());
        Ok(())
    }

    #[test]
    fn test_get_error() {
        let error = get("foo").unwrap_err();
        assert_eq!("unsupported repository for 'foo'", error.to_string());
    }

    #[test]
    #[cfg(feature = "theseus")]
    fn test_get_theseus_postgresql_binaries() {
        assert!(get(theseus::URL).is_ok());
    }

    #[test]
    #[cfg(feature = "zonky")]
    fn test_get_zonky_postgresql_binaries() {
        assert!(get(zonky::URL).is_ok());
    }
}
