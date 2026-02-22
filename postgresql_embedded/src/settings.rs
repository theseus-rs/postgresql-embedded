use crate::error::{Error, Result};
use postgresql_archive::VersionReq;
#[cfg(feature = "bundled")]
use postgresql_archive::{ExactVersionReq, Version};
use rand::RngExt;
use rand::distr::Alphanumeric;
use std::collections::HashMap;
use std::env;
use std::env::{current_dir, home_dir};
use std::ffi::OsString;
use std::path::PathBuf;
#[cfg(feature = "bundled")]
use std::sync::LazyLock;
use std::time::Duration;
use url::Url;

#[cfg(feature = "bundled")]
#[expect(clippy::unwrap_used)]
pub(crate) static ARCHIVE_VERSION: LazyLock<VersionReq> = LazyLock::new(|| {
    let version_string = include_str!(concat!(std::env!("OUT_DIR"), "/postgresql.version"));
    let version = Version::parse(version_string).unwrap();
    let version_req = version.exact_version_req().unwrap();
    tracing::debug!("Bundled installation archive version {version_string}");
    version_req
});

#[cfg(feature = "bundled")]
pub(crate) const ARCHIVE: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/postgresql.tar.gz"));

/// `PostgreSQL` superuser
pub const BOOTSTRAP_SUPERUSER: &str = "postgres";
/// `PostgreSQL` database
pub const BOOTSTRAP_DATABASE: &str = "postgres";

/// Database settings
#[derive(Clone, Debug, PartialEq)]
pub struct Settings {
    /// URL for the releases location of the `PostgreSQL` installation archives
    pub releases_url: String,
    /// Version requirement of `PostgreSQL` to install
    pub version: VersionReq,
    /// `PostgreSQL` installation directory
    pub installation_dir: PathBuf,
    /// `PostgreSQL` password file
    pub password_file: PathBuf,
    /// `PostgreSQL` data directory
    pub data_dir: PathBuf,
    /// `PostgreSQL` host
    pub host: String,
    /// `PostgreSQL` port
    pub port: u16,
    /// `PostgreSQL` user name
    pub username: String,
    /// `PostgreSQL` password
    pub password: String,
    /// Temporary database
    pub temporary: bool,
    /// Command execution Timeout
    pub timeout: Option<Duration>,
    /// Server configuration options
    pub configuration: HashMap<String, String>,
    /// Skip installation and inferrence of the installation dir. Trust what the user provided.
    pub trust_installation_dir: bool,
    /// Unix socket directory. When set, the server will listen on a Unix socket in this directory
    /// in addition to (or instead of) TCP/IP. Unix-only; ignored on Windows.
    pub socket_dir: Option<PathBuf>,
}

/// Settings implementation
impl Settings {
    /// Create a new instance of [`Settings`]
    pub fn new() -> Self {
        let home_dir = home_dir().unwrap_or_else(|| env::current_dir().unwrap_or_default());
        let password_file_name = ".pgpass";
        let password_file = if let Ok(dir) = tempfile::tempdir() {
            dir.keep().join(password_file_name)
        } else {
            let current_dir = current_dir().unwrap_or(PathBuf::from("."));
            current_dir.join(password_file_name)
        };
        let data_dir = if let Ok(dir) = tempfile::tempdir() {
            dir.keep()
        } else {
            let temp_dir: String = rand::rng()
                .sample_iter(&Alphanumeric)
                .take(16)
                .map(char::from)
                .collect();

            let data_dir = current_dir().unwrap_or(PathBuf::from("."));
            data_dir.join(temp_dir)
        };

        let password = rand::rng()
            .sample_iter(&Alphanumeric)
            .take(16)
            .map(char::from)
            .collect();

        #[cfg(feature = "theseus")]
        let releases_url = postgresql_archive::configuration::theseus::URL.to_string();
        #[cfg(not(feature = "theseus"))]
        let releases_url = String::new();

        Self {
            releases_url,
            version: default_version(),
            installation_dir: home_dir.join(".theseus").join("postgresql"),
            password_file,
            data_dir,
            host: "localhost".to_string(),
            port: 0,
            username: BOOTSTRAP_SUPERUSER.to_string(),
            password,
            temporary: true,
            timeout: Some(Duration::from_secs(5)),
            configuration: HashMap::new(),
            trust_installation_dir: false,
            socket_dir: None,
        }
    }

    /// Returns the binary directory for the configured `PostgreSQL` installation.
    #[must_use]
    pub fn binary_dir(&self) -> PathBuf {
        self.installation_dir.join("bin")
    }

    /// Return the `PostgreSQL` URL for the given database name.
    ///
    /// When `socket_dir` is set, the URL will use the Unix socket path
    /// (e.g. `postgresql://user:pass@localhost:5432/db?host=%2Fpath%2Fto%2Fsocket`).
    /// When `socket_dir` is `None`, a standard TCP URL is returned.
    pub fn url<S: AsRef<str>>(&self, database_name: S) -> String {
        match &self.socket_dir {
            Some(socket_dir) => {
                let socket_str = socket_dir.to_string_lossy();
                let encoded: String =
                    url::form_urlencoded::byte_serialize(socket_str.as_bytes()).collect();
                format!(
                    "postgresql://{}:{}@{}:{}/{}?host={}",
                    self.username,
                    self.password,
                    self.host,
                    self.port,
                    database_name.as_ref(),
                    encoded
                )
            }
            None => {
                format!(
                    "postgresql://{}:{}@{}:{}/{}",
                    self.username,
                    self.password,
                    self.host,
                    self.port,
                    database_name.as_ref()
                )
            }
        }
    }

    /// Create a new instance of [`Settings`] from the given URL.
    ///
    /// # Errors
    ///
    /// Returns an error if the URL is invalid.
    pub fn from_url<S: AsRef<str>>(url: S) -> Result<Self> {
        let parsed_url = match Url::parse(url.as_ref()) {
            Ok(parsed_url) => parsed_url,
            Err(error) => {
                return Err(Error::InvalidUrl {
                    url: url.as_ref().to_string(),
                    message: error.to_string(),
                });
            }
        };
        let query_parameters: HashMap<String, String> =
            parsed_url.query_pairs().into_owned().collect();
        let mut settings = Self::default();

        if let Some(releases_url) = query_parameters.get("releases_url") {
            settings.releases_url = releases_url.to_string();
        }
        if let Some(version) = query_parameters.get("version") {
            settings.version = VersionReq::parse(version)?;
        }
        if let Some(installation_dir) = query_parameters.get("installation_dir") {
            settings.installation_dir = PathBuf::from(installation_dir);
        }
        if let Some(password_file) = query_parameters.get("password_file") {
            settings.password_file = PathBuf::from(password_file);
        }
        if let Some(data_dir) = query_parameters.get("data_dir") {
            settings.data_dir = PathBuf::from(data_dir);
        }
        if let Some(host) = parsed_url.host() {
            settings.host = host.to_string();
        }
        if let Some(port) = parsed_url.port() {
            settings.port = port;
        }
        if !parsed_url.username().is_empty() {
            settings.username = parsed_url.username().to_string();
        }
        if let Some(password) = parsed_url.password() {
            settings.password = password.to_string();
        }
        if let Some(temporary) = query_parameters.get("temporary") {
            settings.temporary = temporary == "true";
        }
        if let Some(timeout) = query_parameters.get("timeout") {
            settings.timeout = match timeout.parse::<u64>() {
                Ok(timeout) => Some(Duration::from_secs(timeout)),
                Err(error) => {
                    return Err(Error::InvalidUrl {
                        url: url.as_ref().to_string(),
                        message: error.to_string(),
                    });
                }
            };
        }
        if let Some(trust_installation_dir) = query_parameters.get("trust_installation_dir") {
            settings.trust_installation_dir = trust_installation_dir == "true";
        }
        if let Some(socket_dir) = query_parameters.get("socket_dir") {
            settings.socket_dir = Some(PathBuf::from(socket_dir));
        }
        let configuration_prefix = "configuration.";
        for (key, value) in &query_parameters {
            if key.starts_with(configuration_prefix)
                && let Some(configuration_key) = key.strip_prefix(configuration_prefix)
            {
                settings
                    .configuration
                    .insert(configuration_key.to_string(), value.to_string());
            }
        }

        Ok(settings)
    }
}

/// Implement the [`Settings`] trait for [`Settings`]
impl postgresql_commands::Settings for Settings {
    fn get_binary_dir(&self) -> PathBuf {
        self.binary_dir().clone()
    }

    fn get_host(&self) -> OsString {
        self.host.parse().expect("host")
    }

    fn get_port(&self) -> u16 {
        self.port
    }

    fn get_username(&self) -> OsString {
        self.username.parse().expect("username")
    }

    fn get_password(&self) -> OsString {
        self.password.parse().expect("password")
    }

    fn get_socket_dir(&self) -> Option<PathBuf> {
        self.socket_dir.clone()
    }
}

/// Default implementation for [`Settings`]
impl Default for Settings {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for constructing [`Settings`] with a fluent API.
///
/// # Examples
///
/// ```no_run
/// use postgresql_embedded::SettingsBuilder;
///
/// let settings = SettingsBuilder::new()
///     .host("127.0.0.1")
///     .port(5433)
///     .username("admin")
///     .password("secret")
///     .temporary(false)
///     .build();
/// ```
///
/// To configure a Unix socket:
///
/// ```no_run
/// use postgresql_embedded::SettingsBuilder;
/// use std::path::PathBuf;
///
/// let settings = SettingsBuilder::new()
///     .socket_dir(PathBuf::from("/tmp/pg_socket"))
///     .build();
/// ```
#[derive(Clone, Debug)]
pub struct SettingsBuilder {
    settings: Settings,
}

impl SettingsBuilder {
    /// Create a new [`SettingsBuilder`] starting from the default [`Settings`].
    #[must_use]
    pub fn new() -> Self {
        Self {
            settings: Settings::new(),
        }
    }

    /// Set the releases URL for downloading PostgreSQL archives.
    #[must_use]
    pub fn releases_url<S: Into<String>>(mut self, releases_url: S) -> Self {
        self.settings.releases_url = releases_url.into();
        self
    }

    /// Set the PostgreSQL version requirement.
    #[must_use]
    pub fn version(mut self, version: VersionReq) -> Self {
        self.settings.version = version;
        self
    }

    /// Set the installation directory.
    #[must_use]
    pub fn installation_dir<P: Into<PathBuf>>(mut self, dir: P) -> Self {
        self.settings.installation_dir = dir.into();
        self
    }

    /// Set the password file path.
    #[must_use]
    pub fn password_file<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.settings.password_file = path.into();
        self
    }

    /// Set the data directory.
    #[must_use]
    pub fn data_dir<P: Into<PathBuf>>(mut self, dir: P) -> Self {
        self.settings.data_dir = dir.into();
        self
    }

    /// Set the host name or IP address.
    #[must_use]
    pub fn host<S: Into<String>>(mut self, host: S) -> Self {
        self.settings.host = host.into();
        self
    }

    /// Set the TCP port number.
    #[must_use]
    pub fn port(mut self, port: u16) -> Self {
        self.settings.port = port;
        self
    }

    /// Set the database username.
    #[must_use]
    pub fn username<S: Into<String>>(mut self, username: S) -> Self {
        self.settings.username = username.into();
        self
    }

    /// Set the database password.
    #[must_use]
    pub fn password<S: Into<String>>(mut self, password: S) -> Self {
        self.settings.password = password.into();
        self
    }

    /// Set whether the database is temporary (cleaned up on drop).
    #[must_use]
    pub fn temporary(mut self, temporary: bool) -> Self {
        self.settings.temporary = temporary;
        self
    }

    /// Set the command execution timeout.
    #[must_use]
    pub fn timeout(mut self, timeout: Option<Duration>) -> Self {
        self.settings.timeout = timeout;
        self
    }

    /// Set server configuration options.
    #[must_use]
    pub fn configuration(mut self, configuration: HashMap<String, String>) -> Self {
        self.settings.configuration = configuration;
        self
    }

    /// Add a single server configuration option.
    #[must_use]
    pub fn config<K: Into<String>, V: Into<String>>(mut self, key: K, value: V) -> Self {
        self.settings.configuration.insert(key.into(), value.into());
        self
    }

    /// Set whether to trust the installation directory as-is.
    #[must_use]
    pub fn trust_installation_dir(mut self, trust: bool) -> Self {
        self.settings.trust_installation_dir = trust;
        self
    }

    /// Set the Unix socket directory. When set, the server will listen on a Unix socket in this directory. This is only
    /// supported on Unix platforms.
    #[must_use]
    pub fn socket_dir<P: Into<PathBuf>>(mut self, dir: P) -> Self {
        self.settings.socket_dir = Some(dir.into());
        self
    }

    /// Consume the builder and return the configured [`Settings`].
    #[must_use]
    pub fn build(self) -> Settings {
        self.settings
    }
}

impl Default for SettingsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Get the default version used if not otherwise specified
#[must_use]
fn default_version() -> VersionReq {
    #[cfg(feature = "bundled")]
    {
        ARCHIVE_VERSION.clone()
    }

    #[cfg(not(feature = "bundled"))]
    {
        VersionReq::STAR
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    #[cfg(feature = "bundled")]
    fn test_archive_version() {
        assert!(!super::ARCHIVE_VERSION.to_string().is_empty());
    }

    #[test]
    fn test_settings_new() {
        let settings = Settings::new();
        assert!(
            !settings
                .installation_dir
                .to_str()
                .unwrap_or_default()
                .is_empty()
        );
        assert!(settings.password_file.ends_with(".pgpass"));
        assert!(!settings.data_dir.to_str().unwrap_or_default().is_empty());
        assert_eq!(0, settings.port);
        assert_eq!(BOOTSTRAP_SUPERUSER, settings.username);
        assert!(!settings.password.is_empty());
        assert_ne!("password", settings.password);
        assert!(settings.binary_dir().ends_with("bin"));
        assert_eq!(
            "postgresql://postgres:password@localhost:0/test",
            settings
                .url("test")
                .replace(settings.password.as_str(), "password")
        );
        assert_eq!(Some(Duration::from_secs(5)), settings.timeout);
        assert!(settings.configuration.is_empty());
        assert!(settings.socket_dir.is_none());
    }

    #[test]
    fn test_settings_url_with_socket_dir() {
        let mut settings = Settings::new();
        settings.username = "user".to_string();
        settings.password = "pass".to_string();
        settings.host = "localhost".to_string();
        settings.port = 5432;
        settings.socket_dir = Some(PathBuf::from("/tmp/pg_socket"));

        assert_eq!(
            "postgresql://user:pass@localhost:5432/test?host=%2Ftmp%2Fpg_socket",
            settings.url("test")
        );
    }

    #[test]
    fn test_settings_from_url() -> Result<()> {
        let base_url = "postgresql://postgres:password@localhost:5432/test";
        let releases_url = "releases_url=https%3A%2F%2Fgithub.com";
        let version = "version=%3D16.4.0";
        let installation_dir = "installation_dir=/tmp/postgresql";
        let password_file = "password_file=/tmp/.pgpass";
        let data_dir = "data_dir=/tmp/data";
        let temporary = "temporary=false";
        let trust_installation_dir = "trust_installation_dir=true";
        let timeout = "timeout=10";
        let configuration = "configuration.max_connections=42";
        let url = format!(
            "{base_url}?{releases_url}&{version}&{installation_dir}&{password_file}&{data_dir}&{temporary}&{trust_installation_dir}&{timeout}&{configuration}"
        );

        let settings = Settings::from_url(url)?;

        assert_eq!("https://github.com", settings.releases_url);
        assert_eq!(VersionReq::parse("=16.4.0")?, settings.version);
        assert_eq!(PathBuf::from("/tmp/postgresql"), settings.installation_dir);
        assert_eq!(PathBuf::from("/tmp/.pgpass"), settings.password_file);
        assert_eq!(PathBuf::from("/tmp/data"), settings.data_dir);
        assert_eq!("localhost", settings.host);
        assert_eq!(5432, settings.port);
        assert_eq!(BOOTSTRAP_SUPERUSER, settings.username);
        assert_eq!("password", settings.password);
        assert!(!settings.temporary);
        assert!(settings.trust_installation_dir);
        assert_eq!(Some(Duration::from_secs(10)), settings.timeout);
        let configuration = HashMap::from([("max_connections".to_string(), "42".to_string())]);
        assert_eq!(configuration, settings.configuration);
        assert!(settings.socket_dir.is_none());
        assert_eq!(base_url, settings.url("test"));

        Ok(())
    }

    #[test]
    fn test_settings_from_url_with_socket_dir() -> Result<()> {
        let url =
            "postgresql://postgres:password@localhost:5432/test?socket_dir=%2Ftmp%2Fpg_socket";
        let settings = Settings::from_url(url)?;

        assert_eq!(Some(PathBuf::from("/tmp/pg_socket")), settings.socket_dir);
        assert_eq!("localhost", settings.host);
        assert_eq!(5432, settings.port);
        assert_eq!(
            "postgresql://postgres:password@localhost:5432/test?host=%2Ftmp%2Fpg_socket",
            settings.url("test")
        );

        Ok(())
    }

    #[test]
    fn test_settings_from_url_invalid_url() {
        assert!(Settings::from_url("^`~").is_err());
    }

    #[test]
    fn test_settings_from_url_invalid_version() {
        assert!(Settings::from_url("postgresql://?version=foo").is_err());
    }

    #[test]
    fn test_settings_from_url_invalid_timeout() {
        assert!(Settings::from_url("postgresql://?timeout=foo").is_err());
    }

    #[test]
    fn test_settings_builder_defaults() {
        let settings = SettingsBuilder::new().build();
        assert_eq!("localhost", settings.host);
        assert_eq!(0, settings.port);
        assert_eq!(BOOTSTRAP_SUPERUSER, settings.username);
        assert!(settings.temporary);
        assert!(settings.socket_dir.is_none());
        assert_eq!(Some(Duration::from_secs(5)), settings.timeout);
    }

    #[test]
    fn test_settings_builder_all_fields() {
        let configuration = HashMap::from([("max_connections".to_string(), "100".to_string())]);
        let settings = SettingsBuilder::new()
            .releases_url("https://example.com")
            .version(VersionReq::STAR)
            .installation_dir("/tmp/install")
            .password_file("/tmp/.pgpass")
            .data_dir("/tmp/data")
            .host("127.0.0.1")
            .port(5433)
            .username("admin")
            .password("secret")
            .temporary(false)
            .timeout(Some(Duration::from_secs(30)))
            .configuration(configuration.clone())
            .trust_installation_dir(true)
            .socket_dir(PathBuf::from("/tmp/pg_socket"))
            .build();

        assert_eq!("https://example.com", settings.releases_url);
        assert_eq!(PathBuf::from("/tmp/install"), settings.installation_dir);
        assert_eq!(PathBuf::from("/tmp/.pgpass"), settings.password_file);
        assert_eq!(PathBuf::from("/tmp/data"), settings.data_dir);
        assert_eq!("127.0.0.1", settings.host);
        assert_eq!(5433, settings.port);
        assert_eq!("admin", settings.username);
        assert_eq!("secret", settings.password);
        assert!(!settings.temporary);
        assert_eq!(Some(Duration::from_secs(30)), settings.timeout);
        assert_eq!(configuration, settings.configuration);
        assert!(settings.trust_installation_dir);
        assert_eq!(Some(PathBuf::from("/tmp/pg_socket")), settings.socket_dir);
    }

    #[test]
    fn test_settings_builder_config_method() {
        let settings = SettingsBuilder::new()
            .config("max_connections", "42")
            .config("shared_buffers", "128MB")
            .build();

        assert_eq!(
            Some(&"42".to_string()),
            settings.configuration.get("max_connections")
        );
        assert_eq!(
            Some(&"128MB".to_string()),
            settings.configuration.get("shared_buffers")
        );
    }

    #[test]
    fn test_settings_builder_socket_dir() {
        let settings = SettingsBuilder::new()
            .socket_dir(PathBuf::from("/tmp/pg_socket"))
            .build();

        assert_eq!(Some(PathBuf::from("/tmp/pg_socket")), settings.socket_dir);
    }

    #[test]
    fn test_settings_builder_default() {
        let builder = SettingsBuilder::default();
        let settings = builder.build();
        assert_eq!("localhost", settings.host);
        assert_eq!(0, settings.port);
    }
}
