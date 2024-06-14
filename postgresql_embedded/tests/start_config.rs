use postgresql_archive::LATEST;
use postgresql_commands::psql::PsqlBuilder;
use postgresql_commands::{CommandBuilder, CommandExecutor};
use postgresql_embedded::{PostgreSQL, Settings};
use std::collections::HashMap;
use test_log::test;

#[test(tokio::test)]
async fn start_config() -> anyhow::Result<()> {
    let configuration = HashMap::from([("max_connections".to_string(), "42".to_string())]);
    let settings = Settings {
        configuration,
        ..Default::default()
    };
    let mut postgresql = PostgreSQL::new(LATEST, settings);

    postgresql.setup().await?;
    postgresql.start().await?;
    let settings = postgresql.settings();

    let mut psql = PsqlBuilder::from(settings)
        .command("SELECT setting FROM pg_settings WHERE name = 'max_connections'")
        .no_psqlrc()
        .no_align()
        .tuples_only()
        .build();
    let (stdout, _stderr) = psql.execute()?;

    assert!(stdout.contains("42"));

    Ok(())
}
