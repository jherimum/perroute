use anyhow::Result;
use perroute_backoffice_api::app::Application;
use perroute_commons::{configuration::settings::Settings, tracing::init_tracing};
use std::fmt::{Debug, Display};
use tap::TapFallible;
use tokio::task::JoinError;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    init_tracing();
    let settings =
        Settings::load().tap_err(|e| tracing::error!("Error loading settings. Error: {e}"))?;
    let application = Application::build(&settings).await.unwrap();
    let application_task = tokio::spawn(application.run());

    tokio::select! {
        o = application_task => report_exit("API", o),
    };
    Ok(())
}

fn report_exit(task_name: &str, outcome: Result<Result<(), impl Debug + Display>, JoinError>) {
    match outcome {
        Ok(Ok(())) => {
            tracing::info!("{} has exited", task_name);
        }
        Ok(Err(e)) => {
            tracing::error!(
                error.cause_chain = ?e,
                error.message = %e,
                "{} failed",
                task_name
            );
        }
        Err(e) => {
            tracing::error!(
                error.cause_chain = ?e,
                error.message = %e,
                "{}' task failed to complete",
                task_name
            );
        }
    };
}
