use postgresql_embedded::{PostgreSQL, Settings, Status};
use std::env;
use std::time::Duration;
use test_log::test;

#[test(tokio::test)]
async fn lifecycle() -> anyhow::Result<()> {
    // Explicitly set PGDATABASE environment variable to verify that the library behavior
    // is not affected by the environment
    unsafe {
        env::set_var("PGDATABASE", "foodb");
    }

    let settings = Settings {
        timeout: Some(Duration::from_secs(30)),
        ..Default::default()
    };
    let mut postgresql = PostgreSQL::new(settings);
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
