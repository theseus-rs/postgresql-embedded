#![forbid(unsafe_code)]
#![forbid(clippy::allow_attributes)]
#![deny(clippy::pedantic)]

use postgresql_archive::configuration::zonky;
use postgresql_archive::VersionReq;
use postgresql_embedded::{PostgreSQL, Result, Settings};

#[tokio::main]
async fn main() -> Result<()> {
    let settings = Settings {
        releases_url: zonky::URL.to_string(),
        version: VersionReq::parse("=16.4.0")?,
        ..Default::default()
    };
    let mut postgresql = PostgreSQL::new(settings);
    postgresql.setup().await?;
    postgresql.start().await?;

    let database_name = "test";
    postgresql.create_database(database_name).await?;
    postgresql.database_exists(database_name).await?;
    postgresql.drop_database(database_name).await?;

    postgresql.stop().await
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_main() -> Result<()> {
        main()
    }
}
