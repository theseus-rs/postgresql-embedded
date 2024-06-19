use postgresql_embedded::{PostgreSQL, Status};
use std::env;
use test_log::test;

#[test(tokio::test)]
async fn lifecycle() -> anyhow::Result<()> {
    // Explicitly set PGDATABASE environment variable to verify that the library behavior
    // is not affected by the environment
    env::set_var("PGDATABASE", "foodb");

    let mut postgresql = PostgreSQL::default();

    postgresql.setup().await?;
    postgresql.start().await?;

    let database_name = "test";
    assert!(!postgresql.database_exists(database_name).await?);
    postgresql.create_database(database_name).await?;
    assert!(postgresql.database_exists(database_name).await?);
    postgresql.drop_database(database_name).await?;

    postgresql.stop().await?;
    assert_eq!(Status::Stopped, postgresql.status());
    Ok(())
}
