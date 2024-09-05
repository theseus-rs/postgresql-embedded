use crate::repository::model::Repository;
#[cfg(feature = "portal-corp")]
use crate::repository::portal_corp::repository::PortalCorp;
#[cfg(feature = "steampipe")]
use crate::repository::steampipe::repository::Steampipe;
#[cfg(feature = "tensor-chord")]
use crate::repository::tensor_chord::repository::TensorChord;
use crate::Error::{PoisonedLock, UnsupportedNamespace};
use crate::Result;
use std::collections::HashMap;
use std::sync::{Arc, LazyLock, Mutex, RwLock};

static REGISTRY: LazyLock<Arc<Mutex<RepositoryRegistry>>> =
    LazyLock::new(|| Arc::new(Mutex::new(RepositoryRegistry::default())));

type NewFn = dyn Fn() -> Result<Box<dyn Repository>> + Send + Sync;

/// Singleton struct to store repositories
struct RepositoryRegistry {
    repositories: HashMap<String, Arc<RwLock<NewFn>>>,
}

impl RepositoryRegistry {
    /// Creates a new repository registry.
    fn new() -> Self {
        Self {
            repositories: HashMap::new(),
        }
    }

    /// Registers a repository. Newly registered repositories take precedence over existing ones.
    fn register(&mut self, namespace: &str, new_fn: Box<NewFn>) {
        let namespace = namespace.to_string();
        self.repositories
            .insert(namespace, Arc::new(RwLock::new(new_fn)));
    }

    /// Gets a repository that supports the specified namespace
    ///
    /// # Errors
    /// * If the namespace is not supported.
    fn get(&self, namespace: &str) -> Result<Box<dyn Repository>> {
        let namespace = namespace.to_string();
        let Some(new_fn) = self.repositories.get(&namespace) else {
            return Err(UnsupportedNamespace(namespace.to_string()));
        };
        let new_function = new_fn
            .read()
            .map_err(|error| PoisonedLock(error.to_string()))?;
        new_function()
    }
}

impl Default for RepositoryRegistry {
    /// Creates a new repository registry with the default repositories registered.
    fn default() -> Self {
        let mut registry = Self::new();
        #[cfg(feature = "portal-corp")]
        {
            registry.register("portal-corp", Box::new(PortalCorp::new));
            let _ = PortalCorp::initialize();
        }
        #[cfg(feature = "steampipe")]
        {
            registry.register("steampipe", Box::new(Steampipe::new));
            let _ = Steampipe::initialize();
        }
        #[cfg(feature = "tensor-chord")]
        {
            registry.register("tensor-chord", Box::new(TensorChord::new));
            let _ = TensorChord::initialize();
        }
        registry
    }
}

/// Registers a repository. Newly registered repositories can override existing ones.
///
/// # Errors
/// * If the registry is poisoned.
pub fn register(namespace: &str, new_fn: Box<NewFn>) -> Result<()> {
    let mut registry = REGISTRY
        .lock()
        .map_err(|error| PoisonedLock(error.to_string()))?;
    registry.register(namespace, new_fn);
    Ok(())
}

/// Gets a repository that supports the specified namespace
///
/// # Errors
/// * If the namespace is not supported.
pub fn get(namespace: &str) -> Result<Box<dyn Repository>> {
    let registry = REGISTRY
        .lock()
        .map_err(|error| PoisonedLock(error.to_string()))?;
    registry.get(namespace)
}

/// Gets the namespaces of the registered repositories.
///
/// # Errors
/// * If the registry is poisoned.
pub fn get_namespaces() -> Result<Vec<String>> {
    let registry = REGISTRY
        .lock()
        .map_err(|error| PoisonedLock(error.to_string()))?;
    Ok(registry.repositories.keys().cloned().collect())
}

/// Gets all the registered repositories.
///
/// # Errors
/// * If the registry is poisoned.
pub fn get_repositories() -> Result<Vec<Box<dyn Repository>>> {
    let mut repositories = Vec::new();
    for namespace in get_namespaces()? {
        let repository = get(&namespace)?;
        repositories.push(repository);
    }
    Ok(repositories)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::AvailableExtension;
    use async_trait::async_trait;
    use semver::{Version, VersionReq};
    use std::path::PathBuf;

    #[derive(Debug)]
    struct TestRepository;

    impl TestRepository {
        #[expect(clippy::new_ret_no_self)]
        #[expect(clippy::unnecessary_wraps)]
        fn new() -> Result<Box<dyn Repository>> {
            Ok(Box::new(Self))
        }
    }

    #[async_trait]
    impl Repository for TestRepository {
        fn name(&self) -> &str {
            "test"
        }

        async fn get_available_extensions(&self) -> Result<Vec<AvailableExtension>> {
            Ok(Vec::new())
        }

        async fn get_archive(
            &self,
            _postgresql_version: &str,
            _name: &str,
            _version: &VersionReq,
        ) -> Result<(Version, Vec<u8>)> {
            Ok((Version::new(1, 0, 0), Vec::new()))
        }

        async fn install(
            &self,
            _name: &str,
            _library_dir: PathBuf,
            _extension_dir: PathBuf,
            _archive: &[u8],
        ) -> Result<Vec<PathBuf>> {
            Ok(Vec::new())
        }
    }

    #[tokio::test]
    async fn test_register() -> Result<()> {
        let namespace = "test";
        register(namespace, Box::new(TestRepository::new))?;
        let repository = get(namespace)?;
        assert_eq!("test", repository.name());
        assert!(repository.get_available_extensions().await.is_ok());
        Ok(())
    }

    #[test]
    fn test_get_error() {
        let error = get("foo").unwrap_err();
        assert_eq!("unsupported namespace 'foo'", error.to_string());
    }

    #[test]
    #[cfg(feature = "portal-corp")]
    fn test_get_portal_corp_extensions() {
        assert!(get("portal-corp").is_ok());
    }

    #[test]
    #[cfg(feature = "steampipe")]
    fn test_get_steampipe_extensions() {
        assert!(get("steampipe").is_ok());
    }

    #[test]
    #[cfg(feature = "tensor-chord")]
    fn test_get_tensor_chord_extensions() {
        assert!(get("tensor-chord").is_ok());
    }

    #[test]
    fn test_get_namespaces() {
        let namespaces = get_namespaces().unwrap();
        #[cfg(feature = "portal-corp")]
        assert!(namespaces.contains(&"portal-corp".to_string()));
        #[cfg(feature = "steampipe")]
        assert!(namespaces.contains(&"steampipe".to_string()));
        #[cfg(feature = "tensor-chord")]
        assert!(namespaces.contains(&"tensor-chord".to_string()));
    }

    #[test]
    fn test_get_repositories() {
        let repositories = get_repositories().unwrap();
        #[cfg(feature = "steampipe")]
        assert!(repositories
            .iter()
            .any(|repository| repository.name() == "steampipe"));
        #[cfg(feature = "tensor-chord")]
        assert!(repositories
            .iter()
            .any(|repository| repository.name() == "tensor-chord"));
    }
}
