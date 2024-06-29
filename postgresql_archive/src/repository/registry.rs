use crate::repository::github::repository::GitHub;
use crate::repository::model::Repository;
use crate::Error::{PoisonedLock, UnsupportedRepository};
use crate::Result;
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex, RwLock};

lazy_static! {
    static ref REGISTRY: Arc<Mutex<RepositoryRegistry>> =
        Arc::new(Mutex::new(RepositoryRegistry::default()));
}

type SupportsFn = dyn Fn(&str) -> bool + Send + Sync;
type NewFn = dyn Fn(&str) -> Result<Box<dyn Repository>> + Send + Sync;

/// Singleton struct to store repositories
#[allow(clippy::type_complexity)]
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
    fn register(&mut self, supports_fn: Box<SupportsFn>, new_fn: Box<NewFn>) {
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
            if supports_function(url) {
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
        registry.register(Box::new(GitHub::supports), Box::new(GitHub::new));
        registry
    }
}

/// Registers a repository. Newly registered repositories can override existing ones.
///
/// # Errors
/// * If the registry is poisoned.
#[allow(dead_code)]
pub fn register(supports_fn: Box<SupportsFn>, new_fn: Box<NewFn>) -> Result<()> {
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
        #[allow(clippy::new_ret_no_self)]
        #[allow(clippy::unnecessary_wraps)]
        fn new(_url: &str) -> Result<Box<dyn Repository>> {
            Ok(Box::new(Self))
        }

        fn supports(url: &str) -> bool {
            url == "https://foo.com"
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
            Box::new(TestRepository::supports),
            Box::new(TestRepository::new),
        )?;
        let url = "https://foo.com";
        let repository = get(url)?;
        assert_eq!("test", repository.name());
        Ok(())
    }

    #[tokio::test]
    async fn test_get_no_host() -> Result<()> {
        let url = "https://";
        let error = get(url).err().unwrap();
        assert_eq!("unsupported repository for 'https://'", error.to_string());
        Ok(())
    }

    #[tokio::test]
    async fn test_get_github() -> Result<()> {
        let url = "https://github.com/theseus-rs/postgresql-binaries";
        let result = get(url);
        assert!(result.is_ok());
        Ok(())
    }
}
