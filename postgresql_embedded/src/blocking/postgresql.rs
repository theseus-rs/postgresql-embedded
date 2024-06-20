use crate::{Result, Settings, Status};
use lazy_static::lazy_static;
use postgresql_archive::Version;
use tokio::runtime::Runtime;

lazy_static! {
    static ref RUNTIME: Runtime = Runtime::new().unwrap();
}

/// `PostgreSQL` server
#[derive(Clone, Debug, Default)]
pub struct PostgreSQL {
    inner: crate::postgresql::PostgreSQL,
}

/// `PostgreSQL` server methods
impl PostgreSQL {
    /// Create a new [`crate::postgresql::PostgreSQL`] instance
    #[must_use]
    pub fn new(version: Version, settings: Settings) -> Self {
        Self {
            inner: crate::postgresql::PostgreSQL::new(version, settings),
        }
    }

    /// Get the [status](Status) of the `PostgreSQL` server
    #[must_use]
    pub fn status(&self) -> Status {
        self.inner.status()
    }

    /// Get the [version](Version) of the `PostgreSQL` server
    #[must_use]
    pub fn version(&self) -> &Version {
        self.inner.version()
    }

    /// Get the [settings](Settings) of the `PostgreSQL` server
    #[must_use]
    pub fn settings(&self) -> &Settings {
        self.inner.settings()
    }

    /// Set up the database by extracting the archive and initializing the database.
    /// If the installation directory already exists, the archive will not be extracted.
    /// If the data directory already exists, the database will not be initialized.
    ///
    /// # Errors
    ///
    /// Returns an error if the setup fails.
    pub fn setup(&mut self) -> Result<()> {
        RUNTIME
            .handle()
            .block_on(async move { self.inner.setup().await })
    }

    /// Start the database and wait for the startup to complete.
    /// If the port is set to `0`, the database will be started on a random port.
    ///
    /// # Errors
    ///
    /// Returns an error if the startup fails.
    pub fn start(&mut self) -> Result<()> {
        RUNTIME
            .handle()
            .block_on(async move { self.inner.start().await })
    }

    /// Stop the database gracefully (smart mode) and wait for the shutdown to complete.
    ///
    /// # Errors
    ///
    /// Returns an error if the shutdown fails.
    pub fn stop(&self) -> Result<()> {
        RUNTIME
            .handle()
            .block_on(async move { self.inner.stop().await })
    }

    /// Create a new database with the given name.
    ///
    /// # Errors
    ///
    /// Returns an error if the database creation fails.
    pub fn create_database<S>(&self, database_name: S) -> Result<()>
    where
        S: AsRef<str> + std::fmt::Debug,
    {
        RUNTIME
            .handle()
            .block_on(async move { self.inner.create_database(database_name).await })
    }

    /// Check if a database with the given name exists.
    ///
    /// # Errors
    ///
    /// Returns an error if the database existence check fails.
    pub fn database_exists<S>(&self, database_name: S) -> Result<bool>
    where
        S: AsRef<str> + std::fmt::Debug,
    {
        RUNTIME
            .handle()
            .block_on(async move { self.inner.database_exists(database_name).await })
    }

    /// Drop a database with the given name.
    ///
    /// # Errors
    ///
    /// Returns an error if the database drop fails.
    pub fn drop_database<S>(&self, database_name: S) -> Result<()>
    where
        S: AsRef<str> + std::fmt::Debug,
    {
        RUNTIME
            .handle()
            .block_on(async move { self.inner.drop_database(database_name).await })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_postgresql() {
        let version = Version::new(16, Some(2), Some(0));
        let postgresql = PostgreSQL::new(version, Settings::default());
        let initial_statuses = [Status::NotInstalled, Status::Installed, Status::Stopped];
        assert!(initial_statuses.contains(&postgresql.status()));
        assert_eq!(postgresql.version(), &version);
    }
}
