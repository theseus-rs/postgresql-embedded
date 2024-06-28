use semver::Version;

/// Matcher for PostgreSQL binaries from <https://github.com/theseus-rs/postgresql-binaries>
///
/// # Arguments
/// * `name` - The name of the asset.
/// * `version` - The version of the asset.
///
/// # Returns
/// * Whether the asset matches.
///
/// # Errors
/// * If the asset matcher fails.
#[allow(clippy::unnecessary_wraps)]
pub fn matcher(name: &str, version: &Version) -> crate::Result<bool> {
    let expected_name = format!("postgresql-{}-{}.tar.gz", version, target_triple::TARGET);
    Ok(name == expected_name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Result;

    #[test]
    fn test_asset_match_success() -> Result<()> {
        let version = Version::parse("16.3.0")?;
        let target = target_triple::TARGET;
        let name = format!("postgresql-{version}-{target}.tar.gz");

        assert!(matcher(name.as_str(), &version)?, "{}", name);
        Ok(())
    }

    #[test]
    fn test_asset_match_errors() -> Result<()> {
        let version = Version::parse("16.3.0")?;
        let target = target_triple::TARGET;
        let names = vec![
            format!("foo-{version}-{target}.tar.gz"),
            format!("postgresql-{target}.tar.gz"),
            format!("postgresql-{version}.tar.gz"),
            format!("postgresql-{version}-{target}.tar"),
            format!("postgresql-{version}-{target}"),
        ];

        for name in names {
            assert!(!matcher(name.as_str(), &version)?, "{}", name);
        }
        Ok(())
    }
}
