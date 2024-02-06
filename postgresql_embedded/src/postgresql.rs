use crate::command::initdb::InitDbBuilder;
use crate::command::pg_ctl::Mode::{Start, Stop};
use crate::command::pg_ctl::PgCtlBuilder;
use crate::command::pg_ctl::ShutdownMode::Fast;
use crate::command::traits::{CommandBuilder, CommandExecutor};
use crate::error::EmbeddedError::{
    ArchiveHashMismatch, ArchiveNotFound, DatabaseInitializationError, DatabaseStartError,
    DatabaseStopError,
};
use crate::error::Result;
use crate::settings::Settings;
use bytes::Bytes;
use postgresql_archive::{extract, get_archive};
use postgresql_archive::{get_version, Version};
use sha2::{Digest, Sha256};
use std::fs::{create_dir_all, remove_dir_all, remove_file};
use std::io::prelude::*;
use std::net::TcpListener;
#[cfg(feature = "bundled")]
use std::ops::Deref;
#[cfg(feature = "bundled")]
use std::str::FromStr;
use tracing::{debug, info};

use crate::command::psql::PsqlBuilder;
use crate::EmbeddedError::{CreateDatabaseError, DatabaseExistsError, DropDatabaseError};

#[cfg(feature = "bundled")]
lazy_static::lazy_static! {
    pub(crate) static ref ARCHIVE_VERSION: Version = {
        let version_string = include_str!(concat!(std::env!("OUT_DIR"), "/postgresql.version"));
        let version = Version::from_str(version_string).unwrap();
        info!("Bundled installation archive version {version}");
        version
    };
}

#[cfg(feature = "bundled")]
pub(crate) const ARCHIVE: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/postgresql.tar.gz"));

/// PostgreSQL status
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Status {
    /// Archive not installed
    NotInstalled,
    /// Installing software from archive
    Installing,
    /// Installation complete; not initialized
    Installed,
    /// Initialization running
    Initializing,
    /// Server starting
    Starting,
    /// Server started
    Started,
    /// Server stopping
    Stopping,
    /// Server initialized and stopped
    Stopped,
}

/// PostgreSQL server
#[derive(Clone, Debug)]
pub struct PostgreSQL {
    status: Status,
    version: Version,
    settings: Settings,
}

/// PostgreSQL server methods
impl PostgreSQL {
    /// Create a new [`PostgreSQL`] instance
    pub fn new(version: Version, settings: Settings) -> Self {
        let mut postgresql = PostgreSQL {
            status: Status::NotInstalled,
            version,
            settings,
        };

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

            postgresql.update_status();
        }

        postgresql
    }

    /// Determine the status of the PostgreSQL server based on the settings
    fn update_status(&mut self) {
        let is_installed = self.is_installed();
        let is_initialized = self.is_initialized();

        if is_installed && is_initialized {
            self.status = Status::Stopped
        } else if is_installed {
            self.status = Status::Installed
        } else {
            self.status = Status::NotInstalled
        }
    }

    /// Get the [status](Status) of the PostgreSQL server
    pub fn status(&self) -> Status {
        self.status
    }

    /// Get the [version](Version) of the PostgreSQL server
    pub fn version(&self) -> &Version {
        &self.version
    }

    /// Get the [settings](Settings) of the PostgreSQL server
    pub fn settings(&self) -> &Settings {
        &self.settings
    }

    /// Check if the PostgreSQL server is installed
    fn is_installed(&self) -> bool {
        if self.version.minor.is_none() || self.version.release.is_none() {
            return false;
        }

        let path = &self.settings.installation_dir;
        path.ends_with(self.version.to_string()) && path.exists()
    }

    /// Check if the PostgreSQL server is initialized
    fn is_initialized(&self) -> bool {
        self.settings.data_dir.join("postgresql.conf").exists()
    }

    /// Set up the database by extracting the archive and initializing the database.
    /// If the installation directory already exists, the archive will not be extracted.
    /// If the data directory already exists, the database will not be initialized.
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
    async fn install(&mut self) -> Result<()> {
        let mut archive_bytes: Option<Bytes> = None;

        debug!("Starting installation process for version {}", self.version);

        #[cfg(feature = "bundled")]
        // If the requested version is the same as the version of the bundled archive, use the bundled
        // archive. This avoids downloading the archive in environments where internet access is
        // restricted or undesirable.
        if ARCHIVE_VERSION.deref() == &self.version {
            debug!("Using bundled installation archive");
            archive_bytes = Some(Bytes::copy_from_slice(ARCHIVE));
        }

        // If the minor and release version are not set, determine the latest version and update the
        // version and installation directory accordingly. This is an optimization to avoid downloading
        // the archive if the latest version is already installed.
        if self.version.minor.is_none() || self.version.release.is_none() {
            let version = get_version(&self.version).await?;
            self.version = version;
            self.settings.installation_dir = self
                .settings
                .installation_dir
                .join(self.version.to_string());

            if self.settings.installation_dir.exists() {
                debug!("Installation directory already exists");
                self.update_status();
                return Ok(());
            }
        }

        if archive_bytes.is_none() {
            let (version, bytes, hash) = get_archive(&self.version).await?;

            let mut hasher = Sha256::new();
            hasher.update(&bytes);
            let archive_hash = hex::encode(hasher.finalize());

            if archive_hash != hash {
                return Err(ArchiveHashMismatch { archive_hash, hash });
            }

            self.version = version;
            archive_bytes = Some(bytes);
        }

        if !self.settings.installation_dir.exists() {
            self.status = Status::Installing;
            create_dir_all(&self.settings.installation_dir)?;

            match archive_bytes {
                Some(bytes) => {
                    extract(&bytes, &self.settings.installation_dir).await?;
                    self.status = Status::Installed;
                }
                None => {
                    self.update_status();
                    return Err(ArchiveNotFound(self.version.to_string()));
                }
            }
        }

        info!(
            "Installed PostgreSQL version {} to {}",
            self.version,
            self.settings.installation_dir.to_string_lossy()
        );

        Ok(())
    }

    /// Initialize the database in the data directory. This will create the necessary files and
    /// directories to start the database.
    async fn initialize(&mut self) -> Result<()> {
        if !self.settings.password_file.exists() {
            let mut file = std::fs::File::create(&self.settings.password_file)?;
            file.write_all(self.settings.password.as_bytes())?;
        }

        debug!(
            "Initializing database {}",
            self.settings.data_dir.to_string_lossy()
        );
        let initdb = InitDbBuilder::new()
            .program_dir(self.settings.binary_dir())
            .pgdata(&self.settings.data_dir)
            .auth("password")
            .pwfile(&self.settings.password_file)
            .username(&self.settings.username)
            .encoding("UTF8");

        self.status = Status::Initializing;
        match self.execute_command(initdb).await {
            Ok((_stdout, _stderr)) => {
                self.status = Status::Stopped;
                info!(
                    "Initialized database {}",
                    self.settings.data_dir.to_string_lossy()
                );
                Ok(())
            }
            Err(error) => {
                self.update_status();
                Err(DatabaseInitializationError(error.into()))
            }
        }
    }

    /// Start the database and wait for the startup to complete.
    /// If the port is set to `0`, the database will be started on a random port.
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
        let options = format!("-F -p {}", self.settings.port);
        let pg_ctl = PgCtlBuilder::new()
            .program_dir(self.settings.binary_dir())
            .mode(Start)
            .pgdata(&self.settings.data_dir)
            .log(start_log)
            .options(options)
            .wait();

        self.status = Status::Starting;
        match self.execute_command(pg_ctl).await {
            Ok((_stdout, _stderr)) => {
                self.status = Status::Started;
                info!(
                    "Started database {} on port {}",
                    self.settings.data_dir.to_string_lossy(),
                    self.settings.port
                );
                Ok(())
            }
            Err(error) => {
                self.update_status();
                Err(DatabaseStartError(error.into()))
            }
        }
    }

    /// Stop the database gracefully (smart mode) and wait for the shutdown to complete.
    pub async fn stop(&mut self) -> Result<()> {
        debug!(
            "Stopping database {}",
            self.settings.data_dir.to_string_lossy()
        );
        let pg_ctl = PgCtlBuilder::new()
            .program_dir(self.settings.binary_dir())
            .mode(Stop)
            .pgdata(&self.settings.data_dir)
            .shutdown_mode(Fast)
            .wait();

        self.status = Status::Stopping;
        match self.execute_command(pg_ctl).await {
            Ok((_stdout, _stderr)) => {
                self.status = Status::Stopped;
                info!(
                    "Stopped database {}",
                    self.settings.data_dir.to_string_lossy()
                );
                Ok(())
            }
            Err(error) => {
                self.update_status();
                Err(DatabaseStopError(error.into()))
            }
        }
    }

    /// Create a new database with the given name.
    pub async fn create_database<S: AsRef<str>>(&mut self, database_name: S) -> Result<()> {
        debug!(
            "Creating database {} for {}:{}",
            database_name.as_ref(),
            self.settings.host,
            self.settings.port
        );
        let psql = PsqlBuilder::new()
            .program_dir(self.settings.binary_dir())
            .command(format!("CREATE DATABASE \"{}\"", database_name.as_ref()))
            .host(&self.settings.host)
            .port(self.settings.port)
            .username(&self.settings.username)
            .no_psqlrc()
            .no_align()
            .tuples_only();

        match self.execute_command(psql).await {
            Ok((_stdout, _stderr)) => {
                info!(
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
    pub async fn database_exists<S: AsRef<str>>(&mut self, database_name: S) -> Result<bool> {
        debug!(
            "Checking if database {} exists for {}:{}",
            database_name.as_ref(),
            self.settings.host,
            self.settings.port
        );
        let psql = PsqlBuilder::new()
            .program_dir(self.settings.binary_dir())
            .command(format!(
                "SELECT 1 FROM pg_database WHERE datname='{}'",
                database_name.as_ref()
            ))
            .host(&self.settings.host)
            .port(self.settings.port)
            .username(&self.settings.username)
            .no_psqlrc()
            .no_align()
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
    pub async fn drop_database<S: AsRef<str>>(&mut self, database_name: S) -> Result<()> {
        debug!(
            "Dropping database {} for {}:{}",
            database_name.as_ref(),
            self.settings.host,
            self.settings.port
        );
        let psql = PsqlBuilder::new()
            .program_dir(self.settings.binary_dir())
            .command(format!(
                "DROP DATABASE IF EXISTS \"{}\"",
                database_name.as_ref()
            ))
            .host(&self.settings.host)
            .port(self.settings.port)
            .username(&self.settings.username)
            .no_psqlrc()
            .no_align()
            .tuples_only();

        match self.execute_command(psql).await {
            Ok((_stdout, _stderr)) => {
                info!(
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
    ) -> Result<(String, String)> {
        let mut command = command_builder.build();
        // TODO: move this into the command builder
        command.env("PGPASSWORD", &self.settings.password);
        command.execute(self.settings.timeout).await
    }

    #[cfg(feature = "tokio")]
    /// Execute a command and return the stdout and stderr as strings.
    async fn execute_command<B: CommandBuilder>(
        &self,
        command_builder: B,
    ) -> Result<(String, String)> {
        let mut command = command_builder.build_tokio();
        // TODO: move this into the command builder
        command.env("PGPASSWORD", &self.settings.password);
        command.execute(self.settings.timeout).await
    }
}

/// Default PostgreSQL server
impl Default for PostgreSQL {
    fn default() -> Self {
        #[cfg(not(feature = "bundled"))]
        let version = postgresql_archive::LATEST;

        #[cfg(feature = "bundled")]
        let version = *ARCHIVE_VERSION;

        let settings = Settings::default();

        Self::new(version, settings)
    }
}

/// Stop the PostgreSQL server and remove the data directory if it is marked as temporary.
impl Drop for PostgreSQL {
    fn drop(&mut self) {
        if self.status == Status::Starting || self.status == Status::Started {
            let mut pg_ctl = PgCtlBuilder::new()
                .program_dir(self.settings.binary_dir())
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
    use test_log::test;

    #[test]
    #[cfg(feature = "bundled")]
    fn test_archive_version() {
        assert!(!super::ARCHIVE_VERSION.to_string().is_empty());
    }
}
