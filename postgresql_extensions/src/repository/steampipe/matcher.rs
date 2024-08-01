use postgresql_archive::Result;
use semver::Version;
use std::collections::HashMap;
use std::env::consts;
use url::Url;

/// Matcher for Steampipe binaries from <https://github.com/turbot/>
///
/// # Errors
/// * If the asset matcher fails.
#[allow(clippy::unnecessary_wraps)]
pub fn matcher(url: &str, name: &str, _version: &Version) -> Result<bool> {
    let Ok(url) = Url::parse(url) else {
        return Ok(false);
    };
    let query_parameters: HashMap<String, String> = url.query_pairs().into_owned().collect();
    let Some(postgresql_version) = query_parameters.get("postgresql_version") else {
        return Ok(false);
    };
    let postgresql_version = match postgresql_version.split_once('.') {
        None => return Ok(false),
        Some((major, _)) => major,
    };
    if !name.starts_with("steampipe_postgres_") {
        return Ok(false);
    }
    let os = get_os();
    let arch = get_arch();
    let suffix = format!(".pg{postgresql_version}.{os}_{arch}.tar.gz");
    Ok(name.ends_with(suffix.as_str()))
}

/// Get the OS name for the Steampipe binary.
fn get_os() -> &'static str {
    match consts::OS {
        "macos" => "darwin",
        _ => "linux",
    }
}

/// Get the architecture name for the Steampipe binary.
fn get_arch() -> &'static str {
    match consts::ARCH {
        "x86_64" => "amd64",
        "aarch64" => "arm64",
        _ => consts::ARCH,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository::steampipe;

    #[test]
    fn test_match_success() -> Result<()> {
        let postgresql_major_version = 15;
        let url = format!(
            "{}?postgresql_version={postgresql_major_version}.7",
            steampipe::URL
        );
        let version = Version::parse("0.12.0")?;
        let os = get_os();
        let arch = get_arch();
        let name =
            format!("steampipe_postgres_csv.pg{postgresql_major_version}.{os}_{arch}.tar.gz");

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
        assert!(!matcher(steampipe::URL, "", &Version::new(0, 0, 0))?);
        Ok(())
    }

    #[test]
    fn test_invalid_version() -> Result<()> {
        let url = format!("{}?postgresql_version=16", steampipe::URL);
        assert!(!matcher(url.as_str(), "", &Version::new(0, 0, 0))?);
        Ok(())
    }
    #[test]
    fn test_match_errors() -> Result<()> {
        let postgresql_major_version = 15;
        let url = format!(
            "{}?postgresql_version={postgresql_major_version}.7",
            steampipe::URL
        );
        let version = Version::parse("0.12.0")?;
        let os = get_os();
        let arch = get_arch();
        let names = vec![
            format!("foo_csv.pg{postgresql_major_version}.{os}_{arch}.tar.gz"),
            format!("steampipe_postgres_csv.pg.{os}_{arch}.tar.gz"),
            format!("steampipe_postgres_csv.pg{postgresql_major_version}.{arch}.tar.gz"),
            format!("steampipe_postgres_csv.pg{postgresql_major_version}.{os}.tar.gz"),
            format!("steampipe_postgres_csv.pg{postgresql_major_version}.{os}_{arch}"),
            format!("steampipe_postgres_csv.pg{postgresql_major_version}.{os}_{arch}.zip"),
        ];

        for name in names {
            assert!(!matcher(url.as_str(), name.as_str(), &version)?, "{}", name);
        }
        Ok(())
    }
}
