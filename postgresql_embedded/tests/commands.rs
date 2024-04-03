use postgresql_commands::pg_dump::PgDumpBuilder;
use postgresql_commands::psql::PsqlBuilder;
use postgresql_commands::{CommandBuilder, CommandExecutor};
use postgresql_embedded::PostgreSQL;
use std::fs;
use tempfile::NamedTempFile;
use test_log::test;

#[test(tokio::test)]
async fn dump_command() -> anyhow::Result<()> {
    let mut postgresql = PostgreSQL::default();

    postgresql.setup().await?;
    postgresql.start().await?;
    let settings = postgresql.settings();

    let database_name = "test";
    postgresql.create_database(database_name).await?;

    let mut psql = PsqlBuilder::from(settings)
        .command("CREATE TABLE person42 (id INTEGER, name VARCHAR(20))")
        .dbname(database_name)
        .no_psqlrc()
        .no_align()
        .tuples_only()
        .build();
    let (_stdout, _stderr) = psql.execute()?;

    let temp_file = NamedTempFile::new()?;
    let file = temp_file.as_ref();
    let mut pgdump = PgDumpBuilder::from(settings)
        .dbname(database_name)
        .schema_only()
        .file(file.to_string_lossy().to_string())
        .build();
    let (_stdout, _stderr) = pgdump.execute()?;

    let contents = fs::read_to_string(file)?;
    assert!(contents.contains("person42"));

    Ok(())
}
