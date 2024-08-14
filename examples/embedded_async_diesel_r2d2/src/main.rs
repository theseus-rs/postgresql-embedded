#![forbid(unsafe_code)]
#![deny(clippy::pedantic)]

use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use postgresql_embedded::{PostgreSQL, Result, Settings, VersionReq};

#[tokio::main]
async fn main() -> Result<()> {
    let settings = Settings {
        version: VersionReq::parse("=16.4.0")?,
        ..Default::default()
    };
    let mut postgresql = PostgreSQL::new(settings);
    postgresql.setup().await?;
    postgresql.start().await?;

    let database_name = "test";
    postgresql.create_database(database_name).await?;
    postgresql.database_exists(database_name).await?;

    {
        let database_url = postgresql.settings().url(database_name);
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let _pool = Pool::builder()
            .test_on_check_out(true)
            .build(manager)
            .expect("Could not build connection pool");
    }

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
