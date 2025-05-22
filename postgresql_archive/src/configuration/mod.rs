use semver::Version;

#[cfg(feature = "theseus")]
pub mod theseus;
#[cfg(feature = "zonky")]
pub mod zonky;

/// Matcher for PostgreSQL binaries from custom GitHub release repositories
/// following the same pattern as <https://github.com/theseus-rs/postgresql-binaries>
///
/// # Errors
/// * If the asset matcher fails.
pub fn matcher(_url: &str, name: &str, version: &Version) -> crate::Result<bool> {
    let target = target_triple::TARGET;
    let expected_name = format!("postgresql-{version}-{target}.tar.gz");
    Ok(name == expected_name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Result;

    #[test]
    fn test_asset_match_success() -> Result<()> {
        let url = "";
        let version = Version::parse("16.4.0")?;
        let target = target_triple::TARGET;
        let name = format!("postgresql-{version}-{target}.tar.gz");

        assert!(matcher(url, name.as_str(), &version)?, "{}", name);
        Ok(())
    }

    #[test]
    fn test_asset_match_errors() -> Result<()> {
        let url = "";
        let version = Version::parse("16.4.0")?;
        let target = target_triple::TARGET;
        let names = vec![
            format!("foo-{version}-{target}.tar.gz"),
            format!("postgresql-{target}.tar.gz"),
            format!("postgresql-{version}.tar.gz"),
            format!("postgresql-{version}-{target}.tar"),
            format!("postgresql-{version}-{target}"),
        ];

        for name in names {
            assert!(!matcher(url, name.as_str(), &version)?, "{}", name);
        }
        Ok(())
    }
}
