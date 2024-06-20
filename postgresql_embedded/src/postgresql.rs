use crate::error::Error::{DatabaseInitializationError, DatabaseStartError, DatabaseStopError};
use crate::error::Result;
use crate::settings::{Settings, BOOTSTRAP_SUPERUSER};
use postgresql_archive::{extract, get_archive};
use postgresql_archive::{get_version, Version};
use postgresql_commands::initdb::InitDbBuilder;
use postgresql_commands::pg_ctl::Mode::{Start, Stop};
use postgresql_commands::pg_ctl::PgCtlBuilder;
use postgresql_commands::pg_ctl::ShutdownMode::Fast;
use postgresql_commands::psql::PsqlBuilder;
#[cfg(feature = "tokio")]
use postgresql_commands::AsyncCommandExecutor;
use postgresql_commands::CommandBuilder;
#[cfg(not(feature = "tokio"))]
use postgresql_commands::CommandExecutor;
use std::fs::{remove_dir_all, remove_file};
use std::io::prelude::*;
use std::net::TcpListener;
#[cfg(feature = "bundled")]
use std::str::FromStr;
use tracing::{debug, instrument};

use crate::Error::{CreateDatabaseError, DatabaseExistsError, DropDatabaseError};

#[cfg(feature = "bundled")]
lazy_static::lazy_static! {
    #[allow(clippy::unwrap_used)]
    pub(crate) static ref ARCHIVE_VERSION: Version = {
        let version_string = include_str!(concat!(std::env!("OUT_DIR"), "/postgresql.version"));
        let version = Version::from_str(version_string).unwrap();
        debug!("Bundled installation archive version {version}");
        version
    };
}

#[cfg(feature = "bundled")]
pub(crate) const ARCHIVE: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/postgresql.tar.gz"));

const PGDATABASE: &str = "PGDATABASE";

/// `PostgreSQL` status
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Status {
    /// Archive not installed
    NotInstalled,
    /// Installation complete; not initialized
    Installed,
    /// Server started
    Started,
    /// Server initialized and stopped
    Stopped,
}

/// `PostgreSQL` server
#[derive(Clone, Debug)]
pub struct PostgreSQL {
    version: Version,
    settings: Settings,
}

/// `PostgreSQL` server methods
impl PostgreSQL {
    /// Create a new [`PostgreSQL`] instance
    #[must_use]
    pub fn new(version: Version, settings: Settings) -> Self {
        let mut postgresql = PostgreSQL { version, settings };

        // If the minor and release version are set, append the version to the installation directory
        // to avoid conflicts with other versions.  This will also facilitate setting the status
        // of the server to the correct initial value.  If the minor and release version are not set,
        // the installation directory will be determined dynamically during the installation process.
        if version.minor.is_some() && version.release.is_some() {
            let path = &postgresql.settings.installation_dir;
            let version_string = version.to_string();

            if !path.ends_with(&version_string) {
                postgresql.settings.installation_dir =
                    postgresql.settings.installation_dir.join(version_string);
            }
        }

        postgresql
    }

    /// Get the default version used if not otherwise specified
    #[must_use]
    pub fn default_version() -> Version {
        #[cfg(feature = "bundled")]
        {
            *ARCHIVE_VERSION
        }

        #[cfg(not(feature = "bundled"))]
        {
            postgresql_archive::LATEST
        }
    }

    /// Get the [status](Status) of the PostgreSQL server
    #[instrument(level = "debug", skip(self))]
    pub fn status(&self) -> Status {
        if self.is_running() {
            Status::Started
        } else if self.is_initialized() {
            Status::Stopped
        } else if self.is_installed() {
            Status::Installed
        } else {
            Status::NotInstalled
        }
    }

    /// Get the [version](Version) of the `PostgreSQL` server
    #[must_use]
    pub fn version(&self) -> &Version {
        &self.version
    }

    /// Get the [settings](Settings) of the `PostgreSQL` server
    #[must_use]
    pub fn settings(&self) -> &Settings {
        &self.settings
    }

    /// Check if the `PostgreSQL` server is installed
    fn is_installed(&self) -> bool {
        if self.version.minor.is_none() || self.version.release.is_none() {
            return false;
        }

        let path = &self.settings.installation_dir;
        path.ends_with(self.version.to_string()) && path.exists()
    }

    /// Check if the `PostgreSQL` server is initialized
    fn is_initialized(&self) -> bool {
        self.settings.data_dir.join("postgresql.conf").exists()
    }

    /// Check if the `PostgreSQL` server is running
    fn is_running(&self) -> bool {
        let pid_file = self.settings.data_dir.join("postmaster.pid");
        pid_file.exists()
    }

    /// Set up the database by extracting the archive and initializing the database.
    /// If the installation directory already exists, the archive will not be extracted.
    /// If the data directory already exists, the database will not be initialized.
    #[instrument(skip(self))]
    pub async fn setup(&mut self) -> Result<()> {
        if !self.is_installed() {
            self.install().await?;
        }

        if !self.is_initialized() {
            self.initialize().await?;
        }

        Ok(())
    }

    /// Install the PostgreSQL server from the archive. If the version minor and/or release are not set,
    /// the latest version will be determined dynamically during the installation process. If the archive
    /// hash does not match the expected hash, an error will be returned. If the installation directory
    /// already exists, the archive will not be extracted. If the archive is not found, an error will be
    /// returned.
    #[instrument(skip(self))]
    async fn install(&mut self) -> Result<()> {
        debug!("Starting installation process for version {}", self.version);

        // If the minor and release version are not set, determine the latest version and update the
        // version and installation directory accordingly. This is an optimization to avoid downloading
        // the archive if the latest version is already installed.
        if self.version.minor.is_none() || self.version.release.is_none() {
            let version = get_version(&self.settings.releases_url, &self.version).await?;
            self.version = version;
            self.settings.installation_dir = self
                .settings
                .installation_dir
                .join(self.version.to_string());
        }

        if self.settings.installation_dir.exists() {
            debug!("Installation directory already exists");
            return Ok(());
        }

        #[cfg(feature = "bundled")]
        // If the requested version is the same as the version of the bundled archive, use the bundled
        // archive. This avoids downloading the archive in environments where internet access is
        // restricted or undesirable.
        let (version, bytes) = if *ARCHIVE_VERSION == self.version {
            debug!("Using bundled installation archive");
            (self.version, bytes::Bytes::copy_from_slice(ARCHIVE))
        } else {
            get_archive(&self.settings().releases_url, &self.version).await?
        };

        #[cfg(not(feature = "bundled"))]
        let (version, bytes) = { get_archive(&self.version).await? };

        self.version = version;
        extract(&bytes, &self.settings.installation_dir).await?;

        debug!(
            "Installed PostgreSQL version {} to {}",
            self.version,
            self.settings.installation_dir.to_string_lossy()
        );

        Ok(())
    }

    /// Initialize the database in the data directory. This will create the necessary files and
    /// directories to start the database.
    #[instrument(skip(self))]
    async fn initialize(&mut self) -> Result<()> {
        if !self.settings.password_file.exists() {
            let mut file = std::fs::File::create(&self.settings.password_file)?;
            file.write_all(self.settings.password.as_bytes())?;
        }

        debug!(
            "Initializing database {}",
            self.settings.data_dir.to_string_lossy()
        );

        let initdb = InitDbBuilder::from(&self.settings)
            .pgdata(&self.settings.data_dir)
            .username(BOOTSTRAP_SUPERUSER)
            .auth("password")
            .pwfile(&self.settings.password_file)
            .encoding("UTF8");

        match self.execute_command(initdb).await {
            Ok((_stdout, _stderr)) => {
                debug!(
                    "Initialized database {}",
                    self.settings.data_dir.to_string_lossy()
                );
                Ok(())
            }
            Err(error) => Err(DatabaseInitializationError(error.into())),
        }
    }

    /// Start the database and wait for the startup to complete.
    /// If the port is set to `0`, the database will be started on a random port.
    #[instrument(skip(self))]
    pub async fn start(&mut self) -> Result<()> {
        if self.settings.port == 0 {
            let listener = TcpListener::bind(("0.0.0.0", 0))?;
            self.settings.port = listener.local_addr()?.port();
        }

        debug!(
            "Starting database {} on port {}",
            self.settings.data_dir.to_string_lossy(),
            self.settings.port
        );
        let start_log = self.settings.data_dir.join("start.log");
        let mut options = Vec::new();
        options.push(format!("-F -p {}", self.settings.port));
        for (key, value) in &self.settings.configuration {
            options.push(format!("-c {key}={value}"));
        }
        let pg_ctl = PgCtlBuilder::from(&self.settings)
            .env(PGDATABASE, "")
            .mode(Start)
            .pgdata(&self.settings.data_dir)
            .log(start_log)
            .options(options.as_slice())
            .wait();

        match self.execute_command(pg_ctl).await {
            Ok((_stdout, _stderr)) => {
                debug!(
                    "Started database {} on port {}",
                    self.settings.data_dir.to_string_lossy(),
                    self.settings.port
                );
                Ok(())
            }
            Err(error) => Err(DatabaseStartError(error.into())),
        }
    }

    /// Stop the database gracefully (smart mode) and wait for the shutdown to complete.
    #[instrument(skip(self))]
    pub async fn stop(&self) -> Result<()> {
        debug!(
            "Stopping database {}",
            self.settings.data_dir.to_string_lossy()
        );
        let pg_ctl = PgCtlBuilder::from(&self.settings)
            .mode(Stop)
            .pgdata(&self.settings.data_dir)
            .shutdown_mode(Fast)
            .wait();

        match self.execute_command(pg_ctl).await {
            Ok((_stdout, _stderr)) => {
                debug!(
                    "Stopped database {}",
                    self.settings.data_dir.to_string_lossy()
                );
                Ok(())
            }
            Err(error) => Err(DatabaseStopError(error.into())),
        }
    }

    /// Create a new database with the given name.
    #[instrument(skip(self))]
    pub async fn create_database<S>(&self, database_name: S) -> Result<()>
    where
        S: AsRef<str> + std::fmt::Debug,
    {
        debug!(
            "Creating database {} for {}:{}",
            database_name.as_ref(),
            self.settings.host,
            self.settings.port
        );
        let psql = PsqlBuilder::from(&self.settings)
            .env(PGDATABASE, "")
            .command(format!("CREATE DATABASE \"{}\"", database_name.as_ref()))
            .username(BOOTSTRAP_SUPERUSER)
            .no_psqlrc();

        match self.execute_command(psql).await {
            Ok((_stdout, _stderr)) => {
                debug!(
                    "Created database {} for {}:{}",
                    database_name.as_ref(),
                    self.settings.host,
                    self.settings.port
                );
                Ok(())
            }
            Err(error) => Err(CreateDatabaseError(error.into())),
        }
    }

    /// Check if a database with the given name exists.
    #[instrument(skip(self))]
    pub async fn database_exists<S>(&self, database_name: S) -> Result<bool>
    where
        S: AsRef<str> + std::fmt::Debug,
    {
        debug!(
            "Checking if database {} exists for {}:{}",
            database_name.as_ref(),
            self.settings.host,
            self.settings.port
        );
        let psql = PsqlBuilder::from(&self.settings)
            .env(PGDATABASE, "")
            .command(format!(
                "SELECT 1 FROM pg_database WHERE datname='{}'",
                database_name.as_ref()
            ))
            .username(BOOTSTRAP_SUPERUSER)
            .no_psqlrc()
            .tuples_only();

        match self.execute_command(psql).await {
            Ok((stdout, _stderr)) => match stdout.trim() {
                "1" => Ok(true),
                _ => Ok(false),
            },
            Err(error) => Err(DatabaseExistsError(error.into())),
        }
    }

    /// Drop a database with the given name.
    #[instrument(skip(self))]
    pub async fn drop_database<S>(&self, database_name: S) -> Result<()>
    where
        S: AsRef<str> + std::fmt::Debug,
    {
        debug!(
            "Dropping database {} for {}:{}",
            database_name.as_ref(),
            self.settings.host,
            self.settings.port
        );
        let psql = PsqlBuilder::from(&self.settings)
            .env(PGDATABASE, "")
            .command(format!(
                "DROP DATABASE IF EXISTS \"{}\"",
                database_name.as_ref()
            ))
            .username(BOOTSTRAP_SUPERUSER)
            .no_psqlrc();

        match self.execute_command(psql).await {
            Ok((_stdout, _stderr)) => {
                debug!(
                    "Dropped database {} for {}:{}",
                    database_name.as_ref(),
                    self.settings.host,
                    self.settings.port
                );
                Ok(())
            }
            Err(error) => Err(DropDatabaseError(error.into())),
        }
    }

    #[cfg(not(feature = "tokio"))]
    /// Execute a command and return the stdout and stderr as strings.
    async fn execute_command<B: CommandBuilder>(
        &self,
        command_builder: B,
    ) -> postgresql_commands::Result<(String, String)> {
        let mut command = command_builder.build();
        command.execute()
    }

    #[cfg(feature = "tokio")]
    /// Execute a command and return the stdout and stderr as strings.
    #[instrument(level = "debug", skip(self, command_builder), fields(program = ?command_builder.get_program()))]
    async fn execute_command<B: CommandBuilder>(
        &self,
        command_builder: B,
    ) -> postgresql_commands::Result<(String, String)> {
        let mut command = command_builder.build_tokio();
        command.execute(self.settings.timeout).await
    }
}

/// Default `PostgreSQL` server
impl Default for PostgreSQL {
    fn default() -> Self {
        let version = PostgreSQL::default_version();
        let settings = Settings::default();
        Self::new(version, settings)
    }
}

/// Stop the `PostgreSQL` server and remove the data directory if it is marked as temporary.
impl Drop for PostgreSQL {
    fn drop(&mut self) {
        if self.status() == Status::Started {
            let mut pg_ctl = PgCtlBuilder::from(&self.settings)
                .mode(Stop)
                .pgdata(&self.settings.data_dir)
                .shutdown_mode(Fast)
                .wait()
                .build();

            let _ = pg_ctl.output();
        }

        if self.settings.temporary {
            let _ = remove_dir_all(&self.settings.data_dir);
            let _ = remove_file(&self.settings.password_file);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    #[cfg(feature = "bundled")]
    fn test_archive_version() {
        assert!(!super::ARCHIVE_VERSION.to_string().is_empty());
    }
}
