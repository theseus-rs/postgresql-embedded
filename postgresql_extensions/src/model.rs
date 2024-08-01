use crate::Result;
use semver::Version;
use serde::{Deserialize, Serialize};
#[cfg(test)]
use std::ffi::OsString;
use std::fmt::Display;
#[cfg(not(feature = "tokio"))]
use std::io::Write;
use std::path::PathBuf;
#[cfg(feature = "tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};

/// A struct representing an available extension.
#[derive(Debug)]
pub struct AvailableExtension {
    namespace: String,
    name: String,
    description: String,
}

impl AvailableExtension {
    /// Creates a new available extension.
    #[must_use]
    pub fn new(namespace: &str, name: &str, description: &str) -> Self {
        Self {
            namespace: namespace.to_string(),
            name: name.to_string(),
            description: description.to_string(),
        }
    }

    /// Gets the namespace of the extension.
    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Gets the name of the extension.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Gets the description of the extension.
    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }
}

impl Display for AvailableExtension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{} {}", self.namespace, self.name, self.description)
    }
}

/// A struct representing an installed configuration.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct InstalledConfiguration {
    extensions: Vec<InstalledExtension>,
}

impl InstalledConfiguration {
    /// Creates a new installed configuration.
    #[must_use]
    pub fn new(extensions: Vec<InstalledExtension>) -> Self {
        Self { extensions }
    }

    /// Reads the configuration from the specified `path`.
    ///
    /// # Errors
    /// * If an error occurs while reading the configuration.
    pub async fn read<P: Into<PathBuf>>(path: P) -> Result<Self> {
        #[cfg(feature = "tokio")]
        {
            let mut file = tokio::fs::File::open(path.into()).await?;
            let mut contents = vec![];
            file.read_to_end(&mut contents).await?;
            let config = serde_json::from_slice(&contents)?;
            Ok(config)
        }
        #[cfg(not(feature = "tokio"))]
        {
            let file = std::fs::File::open(path.into())?;
            let reader = std::io::BufReader::new(file);
            let config = serde_json::from_reader(reader)?;
            Ok(config)
        }
    }

    /// Writes the configuration to the specified `path`.
    ///
    /// # Errors
    /// * If an error occurs while writing the configuration.
    pub async fn write<P: Into<PathBuf>>(&self, path: P) -> Result<()> {
        let content = serde_json::to_string_pretty(&self)?;

        #[cfg(feature = "tokio")]
        {
            let mut file = tokio::fs::File::create(path.into()).await?;
            file.write_all(content.as_bytes()).await?;
        }
        #[cfg(not(feature = "tokio"))]
        {
            let mut file = std::fs::File::create(path.into())?;
            file.write_all(content.as_bytes())?;
        }
        Ok(())
    }

    /// Gets the extensions of the configuration.
    #[must_use]
    pub fn extensions(&self) -> &Vec<InstalledExtension> {
        &self.extensions
    }

    /// Gets the extensions of the configuration.
    #[must_use]
    pub fn extensions_mut(&mut self) -> &mut Vec<InstalledExtension> {
        &mut self.extensions
    }
}

/// A struct representing an installed extension.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct InstalledExtension {
    namespace: String,
    name: String,
    version: Version,
    files: Vec<PathBuf>,
}

impl InstalledExtension {
    /// Creates a new installed extension.
    #[must_use]
    pub fn new(namespace: &str, name: &str, version: Version, files: Vec<PathBuf>) -> Self {
        Self {
            namespace: namespace.to_string(),
            name: name.to_string(),
            version,
            files,
        }
    }

    /// Gets the namespace of the extension.
    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Gets the name of the extension.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Gets the version of the extension.
    #[must_use]
    pub fn version(&self) -> &Version {
        &self.version
    }

    /// Gets the files of the extension.
    #[must_use]
    pub fn files(&self) -> &Vec<PathBuf> {
        &self.files
    }
}

impl Display for InstalledExtension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.namespace, self.name, self.version)
    }
}

#[cfg(test)]
pub struct TestSettings;

#[cfg(test)]
impl postgresql_commands::Settings for TestSettings {
    fn get_binary_dir(&self) -> PathBuf {
        PathBuf::from(".")
    }

    fn get_host(&self) -> OsString {
        "localhost".into()
    }

    fn get_port(&self) -> u16 {
        5432
    }

    fn get_username(&self) -> OsString {
        "postgres".into()
    }

    fn get_password(&self) -> OsString {
        "password".into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use postgresql_commands::Settings;

    #[test]
    fn test_settings() {
        let settings = TestSettings;
        assert_eq!(settings.get_binary_dir(), PathBuf::from("."));
        assert_eq!(settings.get_host(), "localhost");
        assert_eq!(settings.get_port(), 5432);
        assert_eq!(settings.get_username(), "postgres");
        assert_eq!(settings.get_password(), "password");
    }

    #[test]
    fn test_available_extension() {
        let available_extension = AvailableExtension::new("namespace", "name", "description");
        assert_eq!(available_extension.namespace(), "namespace");
        assert_eq!(available_extension.name(), "name");
        assert_eq!(available_extension.description(), "description");
        assert_eq!(
            available_extension.to_string(),
            "namespace:name description"
        );
    }

    #[test]
    fn test_installed_configuration() {
        let installed_configuration = InstalledConfiguration::new(vec![]);
        assert!(installed_configuration.extensions.is_empty());
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_installed_configuration_io() -> Result<()> {
        let temp_file = tempfile::NamedTempFile::new()?;
        let file = temp_file.as_ref();
        let extensions = vec![InstalledExtension::new(
            "namespace",
            "name",
            Version::new(1, 0, 0),
            vec![PathBuf::from("file")],
        )];
        let expected_configuration = InstalledConfiguration::new(extensions);
        expected_configuration.write(file).await?;
        let configuration = InstalledConfiguration::read(file).await?;
        assert_eq!(expected_configuration, configuration);
        tokio::fs::remove_file(file).await?;
        Ok(())
    }

    #[test]
    fn test_installed_extension() {
        let installed_extension = InstalledExtension::new(
            "namespace",
            "name",
            Version::new(1, 0, 0),
            vec![PathBuf::from("file")],
        );
        assert_eq!(installed_extension.namespace(), "namespace");
        assert_eq!(installed_extension.name(), "name");
        assert_eq!(installed_extension.version(), &Version::new(1, 0, 0));
        assert_eq!(installed_extension.files(), &vec![PathBuf::from("file")]);
        assert_eq!(installed_extension.to_string(), "namespace:name:1.0.0");
    }
}
