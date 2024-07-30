use postgresql_archive::Result;
use semver::Version;
use std::env::consts;

/// Matcher for Steampipe binaries from <https://github.com/turbot/>
///
/// # Errors
/// * If the asset matcher fails.
#[allow(clippy::unnecessary_wraps)]
pub fn matcher(name: &str, _version: &Version) -> Result<bool> {
    if !name.starts_with("steampipe_postgres_") {
        return Ok(false);
    }
    // TODO: Add support for using the installed PostgreSQL version.
    let postgresql_version = 15;
    let os = match consts::OS {
        "macos" => "darwin",
        _ => "linux",
    };
    let arch = match consts::ARCH {
        "x86_64" => "amd64",
        "aarch64" => "arm64",
        _ => consts::ARCH,
    };
    let suffix = format!(".pg{postgresql_version}.{os}_{arch}.tar.gz");
    Ok(name.ends_with(suffix.as_str()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_asset_match_success() -> Result<()> {
        let version = Version::parse("0.12.0")?;
        let postgresql_version = 15;
        let os = match consts::OS {
            "macos" => "darwin",
            _ => "linux",
        };
        let arch = match consts::ARCH {
            "x86_64" => "amd64",
            "aarch64" => "arm64",
            _ => consts::ARCH,
        };
        let name = format!("steampipe_postgres_csv.pg{postgresql_version}.{os}_{arch}.tar.gz");

        assert!(matcher(name.as_str(), &version)?, "{}", name);
        Ok(())
    }

    #[test]
    fn test_asset_match_errors() -> Result<()> {
        let version = Version::parse("0.12.0")?;
        let postgresql_version = 15;
        let os = match consts::OS {
            "macos" => "darwin",
            _ => "linux",
        };
        let arch = match consts::ARCH {
            "x86_64" => "amd64",
            "aarch64" => "arm64",
            _ => consts::ARCH,
        };
        let names = vec![
            format!("foo_csv.pg{postgresql_version}.{os}_{arch}.tar.gz"),
            format!("steampipe_postgres_csv.pg.{os}_{arch}.tar.gz"),
            format!("steampipe_postgres_csv.pg{postgresql_version}.{arch}.tar.gz"),
            format!("steampipe_postgres_csv.pg{postgresql_version}.{os}.tar.gz"),
            format!("steampipe_postgres_csv.pg{postgresql_version}.{os}_{arch}"),
            format!("steampipe_postgres_csv.pg{postgresql_version}.{os}_{arch}.zip"),
        ];

        for name in names {
            assert!(!matcher(name.as_str(), &version)?, "{}", name);
        }
        Ok(())
    }
}
