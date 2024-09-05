use postgresql_archive::Result;
use regex::Regex;
use semver::Version;
use std::collections::HashMap;
use std::env::consts;
use url::Url;

/// .tar.gz asset matcher that matches the asset name to the postgresql major version, target triple
/// or OS/CPU architecture.
///
/// # Errors
/// * If the asset matcher fails.
pub fn tar_gz_matcher(url: &str, name: &str, version: &Version) -> Result<bool> {
    if !matcher(url, name, version)? {
        return Ok(false);
    }

    Ok(name.ends_with(".tar.gz"))
}

/// .zip asset matcher that matches the asset name to the postgresql major version, target triple or
/// OS/CPU architecture.
///
/// # Errors
/// * If the asset matcher fails.
#[expect(clippy::case_sensitive_file_extension_comparisons)]
pub fn zip_matcher(url: &str, name: &str, version: &Version) -> Result<bool> {
    if !matcher(url, name, version)? {
        return Ok(false);
    }

    Ok(name.ends_with(".zip"))
}

/// Default asset matcher that matches the asset name to the postgresql major version, target triple
/// or OS/CPU architecture.
///
/// # Errors
/// * If the asset matcher fails.
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

    let postgresql_version = format!("pg{postgresql_major_version}");
    let postgresql_version_re = regex(postgresql_version.as_str())?;
    if !postgresql_version_re.is_match(name) {
        return Ok(false);
    }

    let target_re = regex(target_triple::TARGET)?;
    if target_re.is_match(name) {
        return Ok(true);
    }

    let os = consts::OS;
    let os_re = regex(os)?;
    let matches_os = match os {
        "macos" => {
            let darwin_re = regex("darwin")?;
            os_re.is_match(name) || darwin_re.is_match(name)
        }
        _ => os_re.is_match(name),
    };

    let arch = consts::ARCH;
    let arch_re = regex(arch)?;
    let matches_arch = match arch {
        "x86_64" => {
            let amd64_re = regex("amd64")?;
            arch_re.is_match(name) || amd64_re.is_match(name)
        }
        "aarch64" => {
            let arm64_re = regex("arm64")?;
            arch_re.is_match(name) || arm64_re.is_match(name)
        }
        _ => arch_re.is_match(name),
    };
    if matches_os && matches_arch {
        return Ok(true);
    }

    Ok(false)
}

/// Creates a new regex for the specified key.
///
/// # Arguments
/// * `key` - The key to create the regex for.
///
/// # Returns
/// * The regex.
///
/// # Errors
/// * If the regex cannot be created.
fn regex(key: &str) -> Result<Regex> {
    let regex = Regex::new(format!(r"[\W_]{key}[\W_]").as_str())?;
    Ok(regex)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_invalid_url() -> Result<()> {
        let url = "^";
        assert!(!matcher(url, "", &Version::new(0, 0, 0))?);
        Ok(())
    }

    #[test]
    fn test_no_version() -> Result<()> {
        assert!(!matcher("https://foo", "", &Version::new(0, 0, 0))?);
        Ok(())
    }

    #[test]
    fn test_invalid_version() -> Result<()> {
        assert!(!matcher(
            "https://foo?postgresql_version=16",
            "",
            &Version::new(0, 0, 0)
        )?);
        Ok(())
    }

    #[test]
    fn test_tar_gz_matcher() -> Result<()> {
        let postgresql_major_version = 16;
        let url = format!("https://foo?postgresql_version={postgresql_major_version}.3");
        let version = Version::parse("1.2.3")?;
        let target = target_triple::TARGET;

        let valid_name = format!("postgresql-pg{postgresql_major_version}-{target}.tar.gz");
        let invalid_name = format!("postgresql-pg{postgresql_major_version}-{target}.zip");
        assert!(
            tar_gz_matcher(url.as_str(), valid_name.as_str(), &version)?,
            "{}",
            valid_name
        );
        assert!(
            !tar_gz_matcher(url.as_str(), invalid_name.as_str(), &version)?,
            "{}",
            invalid_name
        );
        Ok(())
    }

    #[test]
    fn test_zip_matcher() -> Result<()> {
        let postgresql_major_version = 16;
        let url = format!("https://foo?postgresql_version={postgresql_major_version}.3");
        let version = Version::parse("1.2.3")?;
        let target = target_triple::TARGET;

        let valid_name = format!("postgresql-pg{postgresql_major_version}-{target}.zip");
        let invalid_name = format!("postgresql-pg{postgresql_major_version}-{target}.tar.gz");
        assert!(
            zip_matcher(url.as_str(), valid_name.as_str(), &version)?,
            "{}",
            valid_name
        );
        assert!(
            !zip_matcher(url.as_str(), invalid_name.as_str(), &version)?,
            "{}",
            invalid_name
        );
        Ok(())
    }

    #[test]
    fn test_matcher_success() -> Result<()> {
        let postgresql_major_version = 16;
        let url = format!("https://foo?postgresql_version={postgresql_major_version}.3");
        let version = Version::parse("1.2.3")?;
        let target = target_triple::TARGET;
        let os = consts::OS;
        let arch = consts::ARCH;
        let names = vec![
            format!("postgresql-pg{postgresql_major_version}-{target}.zip"),
            format!("postgresql-pg{postgresql_major_version}-{os}-{arch}.zip"),
            format!("postgresql-pg{postgresql_major_version}-{target}.tar.gz"),
            format!("postgresql-pg{postgresql_major_version}-{os}-{arch}.tar.gz"),
            format!("foo.{target}.pg{postgresql_major_version}.tar.gz"),
            format!("foo.{os}.{arch}.pg{postgresql_major_version}.tar.gz"),
            format!("foo-{arch}-{os}-pg{postgresql_major_version}.tar.gz"),
            format!("foo_{arch}_{os}_pg{postgresql_major_version}.tar.gz"),
        ];

        for name in names {
            assert!(matcher(url.as_str(), name.as_str(), &version)?, "{}", name);
        }
        Ok(())
    }

    #[test]
    fn test_matcher_errors() -> Result<()> {
        let postgresql_major_version = 16;
        let url = format!("https://foo?postgresql_version={postgresql_major_version}.3");
        let version = Version::parse("1.2.3")?;
        let target = target_triple::TARGET;
        let os = consts::OS;
        let arch = consts::ARCH;
        let names = vec![
            format!("foo-pg{postgresql_major_version}.tar.gz"),
            format!("foo-{target}.tar.gz"),
            format!("foo-pg{postgresql_major_version}-{os}.tar.gz"),
            format!("foo-pg{postgresql_major_version}-{arch}.tar.gz"),
            format!("foo-pg{postgresql_major_version}{os}-{arch}.tar"),
            format!("foo-pg{postgresql_major_version}-{os}{arch}.tar.gz"),
        ];

        for name in names {
            assert!(!matcher(url.as_str(), name.as_str(), &version)?, "{}", name);
        }
        Ok(())
    }
}
