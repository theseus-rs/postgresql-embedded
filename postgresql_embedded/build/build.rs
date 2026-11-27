#[cfg(feature = "bundled")]
mod bundle;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "bundled")]
    bundle::stage_postgresql_archive().await?;
    Ok(())
}
