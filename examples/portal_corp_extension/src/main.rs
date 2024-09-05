#![forbid(unsafe_code)]
#![forbid(clippy::allow_attributes)]
#![deny(clippy::pedantic)]

use anyhow::Result;
use indoc::indoc;
use pgvector::Vector;
use sqlx::{PgPool, Row};
use tracing::info;

use postgresql_embedded::{PostgreSQL, Settings, VersionReq};

/// Example of how to install and configure the PortalCorp pgvector extension.
///
/// See: <https://github.com/pgvector/pgvector?tab=readme-ov-file#getting-started>
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().compact().init();

    info!("Installing PostgreSQL");
    let settings = Settings {
        version: VersionReq::parse("=16.4.0")?,
        ..Default::default()
    };
    let mut postgresql = PostgreSQL::new(settings);
    postgresql.setup().await?;

    info!("Installing the vector extension from PortalCorp");
    postgresql_extensions::install(
        postgresql.settings(),
        "portal-corp",
        "pgvector_compiled",
        &VersionReq::parse("=0.16.12")?,
    )
    .await?;

    info!("Starting PostgreSQL");
    postgresql.start().await?;

    let database_name = "vector-example";
    info!("Creating database {database_name}");
    postgresql.create_database(database_name).await?;

    info!("Configuring extension");
    let settings = postgresql.settings();
    let database_url = settings.url(database_name);
    let pool = PgPool::connect(database_url.as_str()).await?;
    pool.close().await;

    info!("Restarting database");
    postgresql.stop().await?;
    postgresql.start().await?;

    info!("Enabling extension");
    let pool = PgPool::connect(database_url.as_str()).await?;
    enable_extension(&pool).await?;

    info!("Creating table");
    create_table(&pool).await?;

    info!("Creating data");
    create_data(&pool).await?;

    info!("Get the nearest neighbors by L2 distance");
    execute_query(
        &pool,
        "SELECT * FROM items ORDER BY embedding <-> '[3,1,2]' LIMIT 5",
    )
    .await?;

    info!("Stopping database");
    postgresql.stop().await?;
    Ok(())
}

async fn enable_extension(pool: &PgPool) -> Result<()> {
    sqlx::query("DROP EXTENSION IF EXISTS vector")
        .execute(pool)
        .await?;
    sqlx::query("CREATE EXTENSION IF NOT EXISTS vector")
        .execute(pool)
        .await?;
    Ok(())
}

async fn create_table(pool: &PgPool) -> Result<()> {
    sqlx::query(indoc! {"
        CREATE TABLE IF NOT EXISTS items (
            id bigserial PRIMARY KEY,
            embedding vector(3) NOT NULL
        )
    "})
    .execute(pool)
    .await?;
    Ok(())
}

async fn create_data(pool: &PgPool) -> Result<()> {
    sqlx::query(indoc! {"
        INSERT INTO items (embedding)
        VALUES
            ('[1,2,3]'),
            ('[4,5,6]')
    "})
    .execute(pool)
    .await?;
    Ok(())
}

async fn execute_query(pool: &PgPool, query: &str) -> Result<()> {
    info!("Query: {query}");
    let rows = sqlx::query(query).fetch_all(pool).await?;
    for row in rows {
        let id: i64 = row.try_get("id")?;
        let embedding: Vector = row.try_get("embedding")?;
        info!("ID: {id}, Embedding: {embedding:?}");
    }
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
