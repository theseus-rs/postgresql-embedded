use postgresql_embedded::{BOOTSTRAP_DATABASE, PostgreSQL, Settings};
use sqlx::{PgPool, Row};
use std::collections::HashMap;
use std::time::Duration;
use test_log::test;

#[test(tokio::test)]
async fn start_config() -> anyhow::Result<()> {
    let configuration = HashMap::from([("max_connections".to_string(), "42".to_string())]);
    let settings = Settings {
        configuration,
        timeout: Some(Duration::from_secs(30)),
        ..Default::default()
    };
    let mut postgresql = PostgreSQL::new(settings);

    postgresql.setup().await?;
    postgresql.start().await?;
    let settings = postgresql.settings();
    let database_url = settings.url(BOOTSTRAP_DATABASE);
    let pool = PgPool::connect(database_url.as_str()).await?;
    let row = sqlx::query("SELECT setting FROM pg_settings WHERE name = $1")
        .bind("max_connections".to_string())
        .fetch_one(&pool)
        .await?;
    let max_connections: String = row.get(0);
    pool.close().await;

    assert_eq!("42".to_string(), max_connections);

    Ok(())
}
