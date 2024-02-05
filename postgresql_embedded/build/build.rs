mod bundle;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    #[cfg(feature = "bundled")]
    bundle::stage_postgresql_archive().await?;
    Ok(())
}
