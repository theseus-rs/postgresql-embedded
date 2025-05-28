use semver::Version;

/// Matcher for PostgreSQL binaries from custom GitHub release repositories
/// following the same pattern as <https://github.com/theseus-rs/postgresql-binaries>
///
/// # Errors
/// * If the asset matcher fails.
/// TODO: support more custom settings
pub fn matcher(_url: &str, name: &str, version: &Version) -> crate::Result<bool> {
    let target = target_triple::TARGET;
    let expected_name = format!("postgresql-{version}-{target}.tar.gz");
    Ok(name == expected_name)
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;
    use crate::{
        Result,
        matcher::{self, registry::SupportsFn},
    };

    #[test]
    fn test_register_custom_repo() -> Result<()> {
        let custom_url = "https://github.com/Owner/Repo";
        let wrapped_url = Arc::new(custom_url.to_string());
        let supports_fn: SupportsFn = Box::new(move |url| Ok(url == wrapped_url.as_str()));
        matcher::registry::register(supports_fn, matcher)?;

        let matcher = matcher::registry::get(custom_url)?;
        let version = Version::new(16, 3, 0);
        let expected_name = format!("postgresql-{}-{}.tar.gz", version, target_triple::TARGET);
        assert!(matcher("", &expected_name, &version)?);

        Ok(())
    }
}
