#![forbid(unsafe_code)]
#![forbid(clippy::allow_attributes)]
#![deny(clippy::pedantic)]

use anyhow::Result;
use axum::extract::State;
use axum::{http::StatusCode, routing::get, Json, Router};
use postgresql_embedded::{PostgreSQL, Settings, VersionReq};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;
use std::time::Duration;
use tokio::net::TcpListener;
use tracing::info;

/// Example of how to use postgresql embedded with axum.
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().compact().init();

    let db_url =
        env::var("DATABASE_URL").unwrap_or_else(|_| "postgresql://postgres@localhost".to_string());
    info!("Installing PostgreSQL");
    let settings = Settings::from_url(&db_url)?;
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

    let database_name = "axum-test";
    info!("Creating database {database_name}");
    postgresql.create_database(database_name).await?;

    info!("Configuring extension");
    let settings = postgresql.settings().clone();
    let database_url = settings.url(database_name);
    let pool = PgPool::connect(database_url.as_str()).await?;
    pool.close().await;

    info!("Restarting database");
    postgresql.stop().await?;
    postgresql.start().await?;

    info!("Setup connection pool");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&database_url)
        .await?;

    info!("Enabling extension");
    enable_extension(&pool).await?;

    info!("Start application");
    let app = Router::new().route("/", get(extensions)).with_state(pool);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("Listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}

async fn enable_extension(pool: &PgPool) -> Result<()> {
    sqlx::query("CREATE EXTENSION IF NOT EXISTS vector")
        .execute(pool)
        .await?;
    Ok(())
}

async fn extensions(State(pool): State<PgPool>) -> Result<Json<Vec<String>>, (StatusCode, String)> {
    sqlx::query_scalar("SELECT name FROM pg_available_extensions ORDER BY name")
        .fetch_all(&pool)
        .await
        .map(Json)
        .map_err(internal_error)
}

fn internal_error<E: std::error::Error>(err: E) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
