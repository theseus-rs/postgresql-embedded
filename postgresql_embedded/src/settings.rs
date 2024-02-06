use home::home_dir;
use rand::distributions::Alphanumeric;
use rand::Rng;
use std::env;
use std::env::current_dir;
use std::path::PathBuf;
use std::time::Duration;

/// Database settings
#[derive(Clone, Debug)]
pub struct Settings {
    /// PostgreSQL installation directory
    pub installation_dir: PathBuf,
    /// PostgreSQL password file
    pub password_file: PathBuf,
    /// PostgreSQL data directory
    pub data_dir: PathBuf,
    /// PostgreSQL host
    pub host: String,
    /// PostgreSQL port
    pub port: u16,
    /// PostgreSQL user name
    pub username: String,
    /// PostgreSQL password
    pub password: String,
    /// Temporary database
    pub temporary: bool,
    /// Command execution Timeout
    pub timeout: Option<Duration>,
}

/// Settings implementation
impl Settings {
    /// Create a new instance of [`Settings`]
    pub fn new() -> Self {
        let home_dir = home_dir().unwrap_or_else(|| env::current_dir().unwrap_or_default());
        let passwword_file_name = ".pgpass";
        let password_file = match tempfile::tempdir() {
            Ok(dir) => dir.into_path().join(passwword_file_name),
            Err(_) => {
                let current_dir = current_dir().unwrap_or(PathBuf::from("."));
                current_dir.join(passwword_file_name)
            }
        };
        let data_dir = match tempfile::tempdir() {
            Ok(dir) => dir.into_path(),
            Err(_) => {
                let temp_dir: String = rand::thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(16)
                    .map(char::from)
                    .collect();

                let data_dir = current_dir().unwrap_or(PathBuf::from("."));
                data_dir.join(temp_dir)
            }
        };
        let password = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(16)
            .map(char::from)
            .collect();

        Self {
            installation_dir: home_dir.join(".theseus").join("postgresql"),
            password_file,
            data_dir,
            host: "localhost".to_string(),
            port: 0,
            username: "postgres".to_string(),
            password,
            temporary: true,
            timeout: Some(Duration::from_secs(5)),
        }
    }

    /// Returns the binary directory for the configured PostgreSQL installation.
    pub fn binary_dir(&self) -> PathBuf {
        self.installation_dir.join("bin")
    }

    /// Return the PostgreSQL URL for the given database name.
    pub fn url<S: AsRef<str>>(&self, database_name: S) -> String {
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

/// Default implementation for [`Settings`]
impl Default for Settings {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_settings_new() -> crate::error::Result<()> {
        let settings = Settings::new();
        assert!(!settings
            .installation_dir
            .to_str()
            .unwrap_or_default()
            .is_empty());
        assert!(settings.password_file.ends_with(".pgpass"));
        assert!(!settings.data_dir.to_str().unwrap_or_default().is_empty());
        assert_eq!(0, settings.port);
        assert_eq!("postgres", settings.username);
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
        Ok(())
    }
}
