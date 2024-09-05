#![forbid(unsafe_code)]
#![forbid(clippy::allow_attributes)]
#![deny(clippy::pedantic)]

use postgresql_archive::blocking::{extract, get_archive};
use postgresql_archive::configuration::theseus;
use postgresql_archive::{Result, VersionReq};

fn main() -> Result<()> {
    let url = theseus::URL;
    let version_req = VersionReq::STAR;
    let (archive_version, archive) = get_archive(url, &version_req)?;
    let out_dir = tempfile::tempdir()?.into_path();
    extract(url, &archive, &out_dir)?;
    println!(
        "PostgreSQL {} extracted to {}",
        archive_version,
        out_dir.to_string_lossy()
    );
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_main() -> Result<()> {
        main()
    }
}
