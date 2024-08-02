use postgresql_archive::Result;
use semver::Version;
use std::collections::HashMap;
use url::Url;

/// Matcher for Portal Corp binaries from <https://github.com/portalcorp>
///
/// # Errors
/// * If the asset matcher fails.
#[allow(clippy::case_sensitive_file_extension_comparisons)]
pub fn matcher(url: &str, name: &str, _version: &Version) -> Result<bool> {
    let Ok(url) = Url::parse(url) else {
        return Ok(false);
    };
    let query_parameters: HashMap<String, String> = url.query_pairs().into_owned().collect();
    let Some(postgresql_version) = query_parameters.get("postgresql_version") else {
        return Ok(false);
    };
    let postgresql_major_version = match postgresql_version.split_once('.') {
        None => return Ok(false),
        Some((major, _)) => major,
    };
    let target = target_triple::TARGET;
    let expected_name = format!("pgvector-{target}-pg{postgresql_major_version}.zip");
    Ok(name == expected_name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository::portal_corp;

    #[test]
    fn test_match_success() -> Result<()> {
        let postgresql_major_version = 16;
        let url = format!(
            "{}?postgresql_version={postgresql_major_version}.6",
            portal_corp::URL
        );
        let version = Version::parse("0.16.12")?;
        let target = target_triple::TARGET;
        let name = format!("pgvector-{target}-pg{postgresql_major_version}.zip");

        assert!(matcher(url.as_str(), name.as_str(), &version)?, "{}", name);
        Ok(())
    }

    #[test]
    fn test_invalid_url() -> Result<()> {
        let url = "^";
        assert!(!matcher(url, "", &Version::new(0, 0, 0))?);
        Ok(())
    }

    #[test]
    fn test_no_version() -> Result<()> {
        assert!(!matcher(portal_corp::URL, "", &Version::new(0, 0, 0))?);
        Ok(())
    }

    #[test]
    fn test_invalid_version() -> Result<()> {
        let url = format!("{}?postgresql_version=16", portal_corp::URL);
        assert!(!matcher(url.as_str(), "", &Version::new(0, 0, 0))?);
        Ok(())
    }

    #[test]
    fn test_match_errors() -> Result<()> {
        let postgresql_major_version = 16;
        let url = format!(
            "{}?postgresql_version={postgresql_major_version}.3",
            portal_corp::URL
        );
        let version = Version::parse("0.16.12")?;
        let target = target_triple::TARGET;
        let names = vec![
            format!("foo-{target}-pg{postgresql_major_version}.zip"),
            format!("pgvector-pg{postgresql_major_version}.zip"),
            format!("pgvector-{target}.zip"),
            format!("pgvector-{target}-pg{postgresql_major_version}.tar.gz"),
        ];

        for name in names {
            assert!(!matcher(url.as_str(), name.as_str(), &version)?, "{}", name);
        }
        Ok(())
    }
}
