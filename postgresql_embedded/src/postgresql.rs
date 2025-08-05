use crate::error::Error::{DatabaseInitializationError, DatabaseStartError, DatabaseStopError};
use crate::error::Result;
use crate::settings::{BOOTSTRAP_DATABASE, BOOTSTRAP_SUPERUSER, Settings};
use std::fmt::Debug;

use postgresql_archive::{ExactVersion, extract};
#[cfg(not(feature = "bundled"))]
use postgresql_archive::{ExactVersionReq, get_archive, get_version};
#[cfg(feature = "tokio")]
use postgresql_commands::AsyncCommandExecutor;
use postgresql_commands::CommandBuilder;
#[cfg(not(feature = "tokio"))]
use postgresql_commands::CommandExecutor;
use postgresql_commands::initdb::InitDbBuilder;
use postgresql_commands::pg_ctl::Mode::{Start, Stop};
use postgresql_commands::pg_ctl::PgCtlBuilder;
use postgresql_commands::pg_ctl::ShutdownMode::Fast;
use semver::Version;
use sqlx::{PgPool, Row};
use std::fs::{read_dir, remove_dir_all, remove_file};
use std::io::prelude::*;
use std::net::TcpListener;
use std::path::PathBuf;
use tracing::{debug, instrument};

use crate::Error::{CreateDatabaseError, DatabaseExistsError, DropDatabaseError};

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
    settings: Settings,
}

/// `PostgreSQL` server methods
impl PostgreSQL {
    /// Create a new [`PostgreSQL`] instance
    #[must_use]
    pub fn new(settings: Settings) -> Self {
        let mut postgresql = PostgreSQL { settings };

        // If an exact version is set, append the version to the installation directory to avoid
        // conflicts with other versions.  This will also facilitate setting the status of the
        // server to the correct initial value.  If the minor and release version are not set, the
        // installation directory will be determined dynamically during the installation process.
        if !postgresql.settings.trust_installation_dir {
            if let Some(version) = postgresql.settings.version.exact_version() {
                let path = &postgresql.settings.installation_dir;
                let version_string = version.to_string();

                if !path.ends_with(&version_string) {
                    postgresql.settings.installation_dir =
                        postgresql.settings.installation_dir.join(version_string);
                }
            }
        }
        postgresql
    }

    /// Get the [status](Status) of the PostgreSQL server
    #[instrument(level = "debug", skip(self))]
    pub fn status(&self) -> Status {
        if self.is_running() {
            Status::Started
        } else if self.is_initialized() {
            Status::Stopped
        } else if self.installed_dir().is_some() {
            Status::Installed
        } else {
            Status::NotInstalled
        }
    }

    /// Get the [settings](Settings) of the `PostgreSQL` server
    #[must_use]
    pub fn settings(&self) -> &Settings {
        &self.settings
    }

    /// Find a directory where `PostgreSQL` server is installed.
    /// This first checks if the installation directory exists and matches the version requirement.
    /// If it doesn't, it will search all the child directories for the latest version that matches the requirement.
    /// If it returns None, we couldn't find a matching installation.
    fn installed_dir(&self) -> Option<PathBuf> {
        if self.settings.trust_installation_dir {
            return Some(self.settings.installation_dir.clone());
        }

        let path = &self.settings.installation_dir;
        let maybe_path_version = path
            .file_name()
            .and_then(|file_name| Version::parse(&file_name.to_string_lossy()).ok());
        // If this directory matches the version requirement, we're done.
        if let Some(path_version) = maybe_path_version {
            if self.settings.version.matches(&path_version) && path.exists() {
                return Some(path.clone());
            }
        }

        // Get all directories in the path as versions.
        let mut versions = read_dir(path)
            .ok()?
            .filter_map(|entry| {
                let Some(entry) = entry.ok() else {
                    // We ignore filesystem errors.
                    return None;
                };
                // Skip non-directories
                if !entry.file_type().ok()?.is_dir() {
                    return None;
                }
                let file_name = entry.file_name();
                let version = Version::parse(&file_name.to_string_lossy()).ok()?;
                if self.settings.version.matches(&version) {
                    Some((version, entry.path()))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        // Sort the versions in descending order i.e. latest version first
        versions.sort_by(|(a, _), (b, _)| b.cmp(a));
        // Get the first matching version as the best match
        versions.first().map(|(_, path)| path.clone())
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
    ///
    /// # Errors
    ///
    /// If the installation fails, an error will be returned.
    #[instrument(skip(self))]
    pub async fn setup(&mut self) -> Result<()> {
        match self.installed_dir() {
            Some(installed_dir) => {
                self.settings.installation_dir = installed_dir;
            }
            None => {
                self.install().await?;
            }
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
        debug!(
            "Starting installation process for version {}",
            self.settings.version
        );

        // If the exact version is not set, determine the latest version and update the version and
        // installation directory accordingly. This is an optimization to avoid downloading the
        // archive if the latest version is already installed.
        #[cfg(not(feature = "bundled"))]
        if self.settings.version.exact_version().is_none() {
            let version = get_version(&self.settings.releases_url, &self.settings.version).await?;
            self.settings.version = version.exact_version_req()?;
            self.settings.installation_dir =
                self.settings.installation_dir.join(version.to_string());
        }
        #[cfg(feature = "bundled")]
        if self.settings.version.exact_version().is_none() {
            panic!(
                "Bundled version should always be set to an exact version e.g. \"=15.4.1\", got - {:?}",
                self.settings.version
            );
        }

        if self.settings.installation_dir.exists() {
            debug!("Installation directory already exists");
            return Ok(());
        }

        let url = &self.settings.releases_url;

        #[cfg(feature = "bundled")]
        // If the requested version is the same as the version of the bundled archive, use the bundled
        // archive. Otherwise don't download the archive, because user expects the bundled archive to be used not the one from internet.
        let (version, bytes) = if *crate::settings::ARCHIVE_VERSION == self.settings.version {
            debug!("Using bundled installation archive");
            (
                self.settings.version.clone(),
                crate::settings::ARCHIVE.to_vec(),
            )
        } else {
            panic!(
                "Bundled version \n\"{:?}\", settings version - \"{:?}\"",
                *crate::settings::ARCHIVE_VERSION,
                self.settings.version
            );
        };

        #[cfg(not(feature = "bundled"))]
        let (version, bytes) = {
            let (version, bytes) = get_archive(url, &self.settings.version).await?;
            (version.exact_version_req()?, bytes)
        };

        self.settings.version = version;
        extract(url, &bytes, &self.settings.installation_dir).await?;

        debug!(
            "Installed PostgreSQL version {} to {}",
            self.settings.version,
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
            Err(error) => Err(DatabaseInitializationError(error.to_string())),
        }
    }

    /// Start the database and wait for the startup to complete.
    /// If the port is set to `0`, the database will be started on a random port.
    ///
    /// # Errors
    ///
    /// If the database fails to start, an error will be returned.
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
            Err(error) => Err(DatabaseStartError(error.to_string())),
        }
    }

    /// Stop the database gracefully (smart mode) and wait for the shutdown to complete.
    ///
    /// # Errors
    ///
    /// If the database fails to stop, an error will be returned.
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
            Err(error) => Err(DatabaseStopError(error.to_string())),
        }
    }

    /// Get a connection pool to the bootstrap database.
    async fn get_pool(&self) -> Result<PgPool> {
        let mut settings = self.settings.clone();
        settings.username = BOOTSTRAP_SUPERUSER.to_string();
        let database_url = settings.url(BOOTSTRAP_DATABASE);
        let pool = PgPool::connect(database_url.as_str()).await?;
        Ok(pool)
    }

    /// Create a new database with the given name.
    ///
    /// # Errors
    ///
    /// If the database creation fails, an error will be returned.
    #[instrument(skip(self))]
    pub async fn create_database<S>(&self, database_name: S) -> Result<()>
    where
        S: AsRef<str> + std::fmt::Debug,
    {
        let database_name = database_name.as_ref();
        debug!(
            "Creating database {database_name} for {host}:{port}",
            host = self.settings.host,
            port = self.settings.port
        );
        let pool = self.get_pool().await?;
        sqlx::query(format!("CREATE DATABASE \"{database_name}\"").as_str())
            .execute(&pool)
            .await
            .map_err(|error| CreateDatabaseError(error.to_string()))?;
        pool.close().await;
        debug!(
            "Created database {database_name} for {host}:{port}",
            host = self.settings.host,
            port = self.settings.port
        );
        Ok(())
    }

    /// Check if a database with the given name exists.
    ///
    /// # Errors
    ///
    /// If the query fails, an error will be returned.
    #[instrument(skip(self))]
    pub async fn database_exists<S>(&self, database_name: S) -> Result<bool>
    where
        S: AsRef<str> + std::fmt::Debug,
    {
        let database_name = database_name.as_ref();
        debug!(
            "Checking if database {database_name} exists for {host}:{port}",
            host = self.settings.host,
            port = self.settings.port
        );
        let pool = self.get_pool().await?;
        let row = sqlx::query("SELECT COUNT(*) FROM pg_database WHERE datname = $1")
            .bind(database_name.to_string())
            .fetch_one(&pool)
            .await
            .map_err(|error| DatabaseExistsError(error.to_string()))?;
        let count: i64 = row.get(0);
        pool.close().await;

        Ok(count == 1)
    }

    /// Drop a database with the given name.
    ///
    /// # Errors
    ///
    /// If the database does not exist or if the drop command fails, an error will be returned.
    #[instrument(skip(self))]
    pub async fn drop_database<S>(&self, database_name: S) -> Result<()>
    where
        S: AsRef<str> + std::fmt::Debug,
    {
        let database_name = database_name.as_ref();
        debug!(
            "Dropping database {database_name} for {host}:{port}",
            host = self.settings.host,
            port = self.settings.port
        );
        let pool = self.get_pool().await?;
        sqlx::query(format!("DROP DATABASE IF EXISTS \"{database_name}\"").as_str())
            .execute(&pool)
            .await
            .map_err(|error| DropDatabaseError(error.to_string()))?;
        pool.close().await;
        debug!(
            "Dropped database {database_name} for {host}:{port}",
            host = self.settings.host,
            port = self.settings.port
        );
        Ok(())
    }

    #[cfg(not(feature = "tokio"))]
    /// Execute a command and return the stdout and stderr as strings.
    #[instrument(level = "debug", skip(self, command_builder), fields(program = ?command_builder.get_program()))]
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
        Self::new(Settings::default())
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
