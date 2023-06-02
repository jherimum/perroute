use anyhow::Result;
use omni_backoffice_api::{
    app::{App, Settings},
    tracing as omni_tracing,
};
use omni_commons::configuration::load_configuration;
use tap::TapFallible;

#[tokio::main]
async fn main() -> Result<()> {
    omni_tracing::init();
    let settings = load_configuration::<Settings>()
        .tap_err(|e| tracing::error!("Error loading settings. Error: {e}"))
        .map_err(anyhow::Error::from)?;

    App::from_settings(&settings)?.init().await?;

    Ok(())
}
