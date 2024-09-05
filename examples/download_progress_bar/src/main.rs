#![forbid(unsafe_code)]
#![forbid(clippy::allow_attributes)]
#![deny(clippy::pedantic)]

use anyhow::Result;
use indicatif::ProgressStyle;
use postgresql_embedded::{PostgreSQL, Settings, VersionReq};
use tracing_indicatif::IndicatifLayer;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{fmt, Registry};

/// Example of how to display a progress bar for the postgresql embedded archive download
#[tokio::main]
async fn main() -> Result<()> {
    let progress_style = ProgressStyle::with_template("{span_child_prefix}{spinner} {span_name} [{elapsed_precise}] [{wide_bar:.green.bold}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")?
        .progress_chars("=> ");
    let indicatif_layer = IndicatifLayer::new().with_progress_style(progress_style);
    let subscriber = Registry::default()
        .with(fmt::Layer::default().with_filter(LevelFilter::INFO))
        .with(indicatif_layer);
    subscriber.init();

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
    postgresql.drop_database(database_name).await?;

    postgresql.stop().await?;
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
