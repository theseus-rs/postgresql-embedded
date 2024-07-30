#![allow(dead_code)]
use crate::model::AvailableExtension;
use crate::{InstalledExtension, Result};
use postgresql_commands::Settings;
use semver::VersionReq;
use std::sync::LazyLock;
use tokio::runtime::Runtime;

static RUNTIME: LazyLock<Runtime> = LazyLock::new(|| Runtime::new().unwrap());

/// Gets the available extensions.
///
/// # Errors
/// * If an error occurs while getting the extensions.
pub fn get_available_extensions() -> Result<Vec<AvailableExtension>> {
    RUNTIME
        .handle()
        .block_on(async move { crate::get_available_extensions().await })
}

/// Gets the installed extensions.
///
/// # Errors
/// * If an error occurs while getting the installed extensions.
pub fn get_installed_extensions(settings: &dyn Settings) -> Result<Vec<InstalledExtension>> {
    RUNTIME
        .handle()
        .block_on(async move { crate::get_installed_extensions(settings).await })
}

/// Installs the extension with the specified `namespace`, `name`, and `version`.
///
/// # Errors
/// * If an error occurs while installing the extension.
pub fn install(
    settings: &dyn Settings,
    namespace: &str,
    name: &str,
    version: &VersionReq,
) -> Result<()> {
    RUNTIME
        .handle()
        .block_on(async move { crate::install(settings, namespace, name, version).await })
}

/// Uninstalls the extension with the specified `namespace` and `name`.
///
/// # Errors
/// * If an error occurs while uninstalling the extension.
pub fn uninstall(settings: &dyn Settings, namespace: &str, name: &str) -> Result<()> {
    RUNTIME
        .handle()
        .block_on(async move { crate::uninstall(settings, namespace, name).await })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TestSettings;

    #[test]
    fn test_get_installed_extensions() -> Result<()> {
        let extensions = get_installed_extensions(&TestSettings)?;
        assert!(extensions.is_empty());
        Ok(())
    }
}
