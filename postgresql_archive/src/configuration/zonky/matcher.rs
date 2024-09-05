use crate::Result;
use semver::Version;
use std::env;

/// Matcher for PostgreSQL binaries from <https://github.com/zonkyio/embedded-postgres-binaries>
///
/// # Errors
/// * If the asset matcher fails.
pub fn matcher(_url: &str, name: &str, version: &Version) -> Result<bool> {
    let os = get_os();
    let arch = get_arch();
    let expected_name = format!("embedded-postgres-binaries-{os}-{arch}-{version}.jar");
    Ok(name == expected_name)
}

/// Returns the operating system of the current system.
pub(crate) fn get_os() -> &'static str {
    match env::consts::OS {
        "macos" => "darwin",
        os => os,
    }
}

/// Returns the architecture of the current system.
pub(crate) fn get_arch() -> &'static str {
    match env::consts::ARCH {
        "arm" => "arm32v7",
        "x86_64" => "amd64",
        "aarch64" => "arm64v8",
        "powerpc64" => "ppc64le",
        "x86" => "i386",
        arch => arch,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Result;

    #[test]
    fn test_asset_match_success() -> Result<()> {
        let url = "";
        let os = get_os();
        let arch = get_arch();
        let version = Version::parse("16.4.0")?;
        let name = format!("embedded-postgres-binaries-{os}-{arch}-{version}.jar");

        assert!(matcher(url, name.as_str(), &version)?, "{}", name);
        Ok(())
    }

    #[test]
    fn test_asset_match_errors() -> Result<()> {
        let url = "";
        let os = get_os();
        let arch = get_arch();
        let version = Version::parse("16.4.0")?;
        let names = vec![
            format!("foo-{os}-{arch}-{version}.jar"),
            format!("embedded-postgres-binaries-{arch}-{version}.jar"),
            format!("embedded-postgres-binaries-{os}-{version}.jar"),
            format!("embedded-postgres-binaries-{os}-{arch}.jar"),
            format!("embedded-postgres-binaries-{os}-{arch}-{version}.zip"),
        ];

        for name in names {
            assert!(!matcher(url, name.as_str(), &version)?, "{}", name);
        }
        Ok(())
    }
}
