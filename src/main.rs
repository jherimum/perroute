use anyhow::Result;
use omni_message::{app::App, configuration::Settings, tracing as omni_tracing};
use tap::TapFallible;

#[tokio::main]
async fn main() -> Result<()> {
    omni_tracing::init();
    let settings = Settings::load()
        .tap_err(|e| tracing::error!("Error loading settings. Error: {e}"))
        .map_err(anyhow::Error::from)?;

    App::from_settings(&settings)?.init().await?;

    Ok(())
}
