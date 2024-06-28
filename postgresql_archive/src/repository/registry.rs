use crate::repository::github::repository::GitHub;
use crate::repository::model::Repository;
use crate::Error::UnsupportedRepository;
use crate::Result;
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex, RwLock};

lazy_static! {
    static ref REGISTRY: Arc<Mutex<RepositoryRegistry>> =
        Arc::new(Mutex::new(RepositoryRegistry::default()));
}

type RepoSupportsFn = Arc<RwLock<dyn Fn(&str) -> bool + Send + Sync>>;
type SupportsFn = Box<dyn Fn(&str) -> bool + Send + Sync>;
type RepoNewFn = Arc<RwLock<dyn Fn(&str) -> Result<Box<dyn Repository>> + Send + Sync>>;
type NewFn = Box<dyn Fn(&str) -> Result<Box<dyn Repository>> + Send + Sync>;

/// Singleton struct to store repositories
struct RepositoryRegistry {
    repositories: Vec<(RepoSupportsFn, RepoNewFn)>,
}

impl RepositoryRegistry {
    /// Creates a new repository registry.
    ///
    /// # Returns
    /// * The repository registry.
    fn new() -> Self {
        Self {
            repositories: Vec::new(),
        }
    }

    /// Registers a repository. Newly registered repositories can override existing ones.
    ///
    /// # Arguments
    /// * `supports_fn` - The function to check if the repository supports the URL.
    /// * `new_fn` - The repository constructor function to register.
    fn register(&mut self, supports_fn: SupportsFn, new_fn: NewFn) {
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
    /// # Arguments
    /// * `url` - The URL to check for support.
    ///
    /// # Returns
    /// * The repository that supports the URL.
    fn get(&self, url: &str) -> Result<Box<dyn Repository>> {
        for (supports_fn, new_fn) in &self.repositories {
            let supports_function = supports_fn.read().unwrap();
            if supports_function(url) {
                let new_function = new_fn.read().unwrap();
                return new_function(url);
            }
        }

        Err(UnsupportedRepository(url.to_string()))
    }
}

impl Default for RepositoryRegistry {
    fn default() -> Self {
        let mut registry = Self::new();
        registry.register(Box::new(GitHub::supports), Box::new(GitHub::new));
        registry
    }
}

/// Registers a repository. Newly registered repositories can override existing ones.
///
/// # Arguments
/// * `supports_fn` - The function to check if the repository supports the URL.
/// * `new_fn` - The repository constructor function to register.
#[allow(dead_code)]
pub fn register(supports_fn: SupportsFn, new_fn: NewFn) {
    let mut registry = REGISTRY.lock().unwrap();
    registry.register(supports_fn, new_fn);
}

/// Gets a repository that supports the specified URL
///
/// # Arguments
/// * `url` - The URL to check for support.
///
/// # Returns
/// * The repository that supports the URL.
///
/// # Errors
/// * If the URL is not supported.
pub fn get(url: &str) -> Result<Box<dyn Repository>> {
    let registry = REGISTRY.lock().unwrap();
    registry.get(url)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_register() -> Result<()> {
        assert!(!REGISTRY.lock().unwrap().repositories.is_empty());
        REGISTRY.lock().unwrap().repositories.truncate(0);
        assert!(REGISTRY.lock().unwrap().repositories.is_empty());

        register(Box::new(GitHub::supports), Box::new(GitHub::new));
        let url = "https://github.com/theseus-rs/postgresql-binaries";
        let result = get(url);
        assert!(result.is_ok());
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
