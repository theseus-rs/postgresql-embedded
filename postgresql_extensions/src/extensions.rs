use crate::model::AvailableExtension;
use crate::repository::registry;
use crate::repository::registry::get_repositories;
use crate::{InstalledConfiguration, InstalledExtension, Result};
use postgresql_commands::pg_config::PgConfigBuilder;
use postgresql_commands::postgres::PostgresBuilder;
#[cfg(feature = "tokio")]
use postgresql_commands::AsyncCommandExecutor;
use postgresql_commands::CommandBuilder;
#[cfg(not(feature = "tokio"))]
use postgresql_commands::CommandExecutor;
use postgresql_commands::Settings;
use regex::Regex;
use semver::VersionReq;
use std::path::PathBuf;
use tracing::{debug, instrument};

const CONFIGURATION_FILE: &str = "postgresql_extensions.json";

/// Gets the available extensions.
///
/// # Errors
/// * If an error occurs while getting the extensions.
#[instrument(level = "debug")]
pub async fn get_available_extensions() -> Result<Vec<AvailableExtension>> {
    let mut extensions = Vec::new();
    for repository in get_repositories()? {
        for extension in repository.get_available_extensions().await? {
            extensions.push(extension);
        }
    }
    Ok(extensions)
}

/// Gets the installed extensions.
///
/// # Errors
/// * If an error occurs while getting the installed extensions.
#[instrument(level = "debug", skip(settings))]
pub async fn get_installed_extensions(settings: &impl Settings) -> Result<Vec<InstalledExtension>> {
    let configuration_file = get_configuration_file(settings).await?;
    if !configuration_file.exists() {
        debug!("No configuration file found: {configuration_file:?}");
        return Ok(Vec::new());
    }

    let configuration = InstalledConfiguration::read(configuration_file).await?;
    let extensions = configuration.extensions();
    Ok(extensions.clone())
}

/// Installs the extension with the specified `namespace`, `name`, and `version`.
///
/// # Errors
/// * If an error occurs while installing the extension.
#[instrument(level = "debug", skip(settings))]
pub async fn install(
    settings: &impl Settings,
    namespace: &str,
    name: &str,
    version: &VersionReq,
) -> Result<()> {
    let extensions = get_installed_extensions(settings).await?;
    if extensions
        .iter()
        .any(|extension| extension.namespace() == namespace && extension.name() == name)
    {
        // Attempt to uninstall the extension first
        uninstall(settings, namespace, name).await?;
    };

    let postgresql_version = get_postgresql_version(settings).await?;
    let repository = registry::get(namespace)?;
    let (version, archive) = repository
        .get_archive(postgresql_version.as_str(), name, version)
        .await?;
    let library_dir = get_library_path(settings).await?;
    let extension_dir = get_extension_path(settings).await?;
    let files = repository
        .install(name, library_dir, extension_dir, &archive)
        .await?;

    let configuration_file = get_configuration_file(settings).await?;
    let mut configuration = if configuration_file.exists() {
        InstalledConfiguration::read(&configuration_file).await?
    } else {
        debug!("No configuration file found: {configuration_file:?}; creating new file");
        InstalledConfiguration::default()
    };
    let installed_extension = InstalledExtension::new(namespace, name, version, files);
    configuration.extensions_mut().push(installed_extension);
    configuration.write(configuration_file).await?;
    Ok(())
}

/// Uninstalls the extension with the specified `namespace` and `name`.
///
/// # Errors
/// * If an error occurs while uninstalling the extension.
#[instrument(level = "debug", skip(settings))]
pub async fn uninstall(settings: &impl Settings, namespace: &str, name: &str) -> Result<()> {
    let configuration_file = get_configuration_file(settings).await?;
    if !configuration_file.exists() {
        debug!("No configuration file found: {configuration_file:?}; nothing to uninstall");
        return Ok(());
    }

    let configuration = &mut InstalledConfiguration::read(&configuration_file).await?;
    let mut extensions = Vec::new();
    for extension in configuration.extensions() {
        if extension.namespace() != namespace || extension.name() != name {
            extensions.push(extension.clone());
        }

        for file in extension.files() {
            if file.exists() {
                debug!("Removing file: {file:?}");
                #[cfg(feature = "tokio")]
                tokio::fs::remove_file(file).await?;
                #[cfg(not(feature = "tokio"))]
                std::fs::remove_file(file)?;
            }
        }
    }

    let configuration = InstalledConfiguration::new(extensions);
    configuration.write(configuration_file).await?;

    Ok(())
}

/// Gets the configuration file.
///
/// # Errors
/// * If an error occurs while getting the configuration file.
async fn get_configuration_file(settings: &dyn Settings) -> Result<PathBuf> {
    let shared_path = get_shared_path(settings).await?;
    let file = shared_path.join(CONFIGURATION_FILE);
    Ok(file)
}

/// Gets the library path.
///
/// # Errors
/// * If an error occurs while getting the library path.
async fn get_library_path(settings: &dyn Settings) -> Result<PathBuf> {
    let command = PgConfigBuilder::from(settings).libdir();
    match execute_command(command).await {
        Ok((stdout, _stderr)) => Ok(PathBuf::from(stdout.trim())),
        Err(error) => {
            debug!("Failed to get library path using pg_config: {error:?}");
            let binary_dir = settings.get_binary_dir();
            let install_dir = if let Some(parent) = binary_dir.parent() {
                parent.to_path_buf()
            } else {
                debug!("Failed to get parent directory of binary directory; defaulting to current directory");
                PathBuf::from(".")
            };
            let library_dir = install_dir.join("lib");
            debug!("Using library directory: {library_dir:?}");
            Ok(library_dir)
        }
    }
}

/// Gets the shared path.
///
/// # Errors
/// * If an error occurs while getting the shared path.
async fn get_shared_path(settings: &dyn Settings) -> Result<PathBuf> {
    let command = PgConfigBuilder::from(settings).sharedir();
    match execute_command(command).await {
        Ok((stdout, _stderr)) => Ok(PathBuf::from(stdout.trim())),
        Err(error) => {
            debug!("Failed to get shared path using pg_config: {error:?}");
            let binary_dir = settings.get_binary_dir();
            let install_dir = if let Some(parent) = binary_dir.parent() {
                parent.to_path_buf()
            } else {
                debug!("Failed to get parent directory of binary directory; defaulting to current directory");
                PathBuf::from(".")
            };
            let share_dir = install_dir.join("share");
            debug!("Using share directory: {share_dir:?}");
            Ok(share_dir)
        }
    }
}

/// Gets the extension path.
///
/// # Errors
/// * If an error occurs while getting the extension path.
async fn get_extension_path(settings: &dyn Settings) -> Result<PathBuf> {
    let shared_path = get_shared_path(settings).await?;
    let extension_path = shared_path.join("extension");
    Ok(extension_path)
}

/// Gets the PostgreSQL version.
///
/// # Errors
/// * If an error occurs while getting the PostgreSQL version.
async fn get_postgresql_version(settings: &dyn Settings) -> Result<String> {
    let command = PostgresBuilder::new()
        .program_dir(settings.get_binary_dir())
        .version();
    let (stdout, _stderr) = execute_command(command).await?;
    let re = Regex::new(r"PostgreSQL\)\s(\d+\.\d+)")?;
    let Some(captures) = re.captures(&stdout) else {
        return Err(regex::Error::Syntax(format!(
            "Failed to obtain postgresql version from {stdout}"
        ))
        .into());
    };
    let Some(version) = captures.get(1) else {
        return Err(regex::Error::Syntax(format!(
            "Failed to match postgresql version from {stdout}"
        ))
        .into());
    };
    let version = version.as_str();
    debug!("Obtained PostgreSQL version from postgres command: {version}");
    Ok(version.to_string())
}

#[cfg(not(feature = "tokio"))]
/// Execute a command and return the stdout and stderr as strings.
#[instrument(level = "debug", skip(command_builder), fields(program = ?command_builder.get_program()))]
async fn execute_command<B: CommandBuilder>(
    command_builder: B,
) -> postgresql_commands::Result<(String, String)> {
    let mut command = command_builder.build();
    command.execute()
}

#[cfg(feature = "tokio")]
/// Execute a command and return the stdout and stderr as strings.
#[instrument(level = "debug", skip(command_builder), fields(program = ?command_builder.get_program()))]
async fn execute_command<B: CommandBuilder>(
    command_builder: B,
) -> postgresql_commands::Result<(String, String)> {
    let mut command = command_builder.build_tokio();
    command.execute(None).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TestSettings;

    #[tokio::test]
    async fn test_get_installed_extensions() -> Result<()> {
        let extensions = get_installed_extensions(&TestSettings).await?;
        assert!(extensions.is_empty());
        Ok(())
    }
}
