use anyhow::Result;
use omni_backoffice_api::app::App;
use omni_commons::{configuration::settings::Settings, tracing::init_tracing};
use tap::TapFallible;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    init_tracing();
    let settings = Settings::load()
        .tap_err(|e| tracing::error!("Error loading settings. Error: {e}"))
        .map_err(anyhow::Error::from)?;

    App::from_settings(&settings).await?.init().await?;

    Ok(())
}
