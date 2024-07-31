#![forbid(unsafe_code)]
#![deny(clippy::pedantic)]

use anyhow::Result;
use postgresql_embedded::{PostgreSQL, Settings, VersionReq};
use sqlx::PgPool;

#[tokio::main]
async fn main() -> Result<()> {
    let settings = Settings {
        version: VersionReq::parse("=16.3.0")?,
        ..Default::default()
    };

    println!("Installing the vector extension from TensorChord");
    postgresql_extensions::install(
        &settings,
        "tensor-chord",
        "pgvecto.rs",
        &VersionReq::parse("=0.3.0")?,
    )
    .await?;

    println!("Installing PostgreSQL");
    let mut postgresql = PostgreSQL::new(settings);
    postgresql.setup().await?;
    postgresql.start().await?;

    let database_name = "vector-example";
    println!("Creating database {database_name}");
    postgresql.create_database(database_name).await?;

    println!("Connecting to database {database_name}");
    let settings = postgresql.settings();
    let database_url = settings.url(database_name);
    let pool = PgPool::connect(database_url.as_str()).await?;

    println!("Configuring the vector extension");
    sqlx::query("CREATE EXTENSION vectors")
        .execute(pool)
        .await?;

    println!("Stopping database");
    postgresql.stop().await?;
    Ok(())
}

#[cfg(target_os = "linux")]
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_main() -> Result<()> {
        main()
    }
}
