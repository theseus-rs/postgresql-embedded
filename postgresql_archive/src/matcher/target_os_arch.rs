use regex::Regex;
use semver::Version;
use std::env;

/// Default asset matcher that matches the asset name to the current target triple or OS/CPU
/// architecture.
///
/// # Errors
/// * If the asset matcher fails.
pub fn matcher(name: &str, _version: &Version) -> crate::Result<bool> {
    if !name.ends_with(".tar.gz") {
        return Ok(false);
    }
    let target_re = regex(target_triple::TARGET)?;
    if target_re.is_match(name) {
        return Ok(true);
    }
    let os = env::consts::OS;
    let os_re = regex(os)?;
    let matches_os = match os {
        "macos" => {
            let darwin_re = regex("darwin")?;
            os_re.is_match(name) || darwin_re.is_match(name)
        }
        _ => os_re.is_match(name),
    };
    let arch = env::consts::ARCH;
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
fn regex(key: &str) -> crate::Result<Regex> {
    let regex = Regex::new(format!(r"[\W_]{key}[\W_]").as_str())?;
    Ok(regex)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Result;

    #[test]
    fn test_asset_match_success() -> Result<()> {
        let version = Version::parse("16.3.0")?;
        let target = target_triple::TARGET;
        let os = env::consts::OS;
        let arch = env::consts::ARCH;
        let names = vec![
            format!("postgresql-16.3.0-{target}.tar.gz"),
            format!("postgresql-16.3.0-{os}-{arch}.tar.gz"),
            format!("foo.{target}.tar.gz"),
            format!("foo.{os}.{arch}.tar.gz"),
            format!("foo-{arch}-{os}.tar.gz"),
        ];

        for name in names {
            assert!(matcher(name.as_str(), &version)?, "{}", name);
        }
        Ok(())
    }

    #[test]
    fn test_asset_match_errors() -> Result<()> {
        let version = Version::parse("16.3.0")?;
        let target = target_triple::TARGET;
        let os = env::consts::OS;
        let arch = env::consts::ARCH;
        let names = vec![
            format!("foo{target}.tar.gz"),
            format!("foo{os}-{arch}.tar.gz"),
            format!("foo-{target}.tar"),
            format!("foo-{os}-{arch}.tar"),
            format!("foo-{os}{arch}.tar.gz"),
        ];

        for name in names {
            assert!(!matcher(name.as_str(), &version)?, "{}", name);
        }
        Ok(())
    }
}
