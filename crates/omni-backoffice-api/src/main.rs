use anyhow::Result;
use omni_backoffice_api::app::{App, Settings};
use omni_commons::{configuration::load_configuration, tracing::init_tracing};
use tap::TapFallible;

#[tokio::main]
async fn main() -> Result<()> {
    init_tracing();
    let settings = load_configuration::<Settings>()
        .tap_err(|e| tracing::error!("Error loading settings. Error: {e}"))
        .map_err(anyhow::Error::from)?;

    App::from_settings(&settings).await?.init().await?;

    Ok(())
}
