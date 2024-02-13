use postgresql_embedded::{PostgreSQL, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let mut postgresql = PostgreSQL::default();
    postgresql.setup().await?;
    postgresql.start().await?;

    let database_name = "test";
    postgresql.create_database(database_name).await?;
    postgresql.database_exists(database_name).await?;
    postgresql.drop_database(database_name).await?;

    postgresql.stop().await
}
