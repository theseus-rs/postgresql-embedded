use semver::Version;

/// Matcher for PostgreSQL binaries from custom GitHub release repositories following the same
/// pattern as <https://github.com/theseus-rs/postgresql-binaries>
///
/// # Errors
/// * If the asset matcher fails.
pub fn matcher(_url: &str, name: &str, version: &Version) -> crate::Result<bool> {
    let target = target_triple::TARGET;
    // TODO: consider relaxing the version format to allow for more flexibility in where the version
    //       and target appear in the filename.
    let expected_name = format!("postgresql-{version}-{target}.tar.gz");
    Ok(name == expected_name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Result, matcher};

    const TEST_URL: &str = "https://github.com/owner/repo";

    #[test]
    fn test_register_custom_repo() -> Result<()> {
        #[expect(clippy::unnecessary_wraps)]
        fn supports_fn(url: &str) -> Result<bool> {
            Ok(url == TEST_URL)
        }
        matcher::registry::register(supports_fn, matcher)?;

        let matcher = matcher::registry::get(TEST_URL)?;
        let version = Version::new(16, 3, 0);
        let expected_name = format!("postgresql-{}-{}.tar.gz", version, target_triple::TARGET);
        assert!(matcher("", &expected_name, &version)?);
        Ok(())
    }
}
