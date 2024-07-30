use postgresql_archive::Result;
use semver::Version;

/// Matcher for Tensor Chord binaries from <https://github.com/tensorchord/>
///
/// # Errors
/// * If the asset matcher fails.
#[allow(clippy::case_sensitive_file_extension_comparisons)]
pub fn matcher(name: &str, version: &Version) -> Result<bool> {
    // TODO: Add support for using the installed PostgreSQL version.
    let postgresql_version = 16;
    let target = target_triple::TARGET;
    let expected_name = format!("vectors-pg{postgresql_version}_{target}_{version}.zip");
    Ok(name == expected_name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_asset_match_success() -> Result<()> {
        let version = Version::parse("0.3.0")?;
        let postgresql_version = 16;
        let target = target_triple::TARGET;
        let name = format!("vectors-pg{postgresql_version}_{target}_{version}.zip");

        assert!(matcher(name.as_str(), &version)?, "{}", name);
        Ok(())
    }

    #[test]
    fn test_asset_match_errors() -> Result<()> {
        let version = Version::parse("0.3.0")?;
        let postgresql_version = 16;
        let target = target_triple::TARGET;
        let names = vec![
            format!("vectors-pg{target}_{version}.zip"),
            format!("vectors-pg{postgresql_version}_{version}.zip"),
            format!("vectors-pg{postgresql_version}_{target}_{version}.tar.gz"),
        ];

        for name in names {
            assert!(!matcher(name.as_str(), &version)?, "{}", name);
        }
        Ok(())
    }
}
