use anyhow::bail;
use perroute_commons::{
    configuration::settings::Settings,
    tracing::init_tracing,
    types::{
        id::Id,
        template::{TemplateData, TemplateError, TemplateRender},
    },
};
use perroute_connectors::{api::DispatchRequest, template::DispatchTemplate, Plugins};
use perroute_messaging::connection::{Config, RecoverableConnection};
use perroute_storage::{
    connection_manager::ConnectionManager,
    models::{
        message_dispatch::{MessageDispatch, MessageDispatchQueryBuilder, MessageDispatchStatus},
        template::Template,
    },
    query::FetchableModel,
};
use sqlx::PgPool;
use std::time::Duration;
use tap::TapFallible;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenv::dotenv().ok();
    init_tracing();
    let settings = Settings::load().tap_err(|e| tracing::error!("Failed to load settings: {e}"))?;
    let pool = ConnectionManager::build_pool(&settings.database)
        .await
        .tap_err(|e| tracing::error!("Failed to build pool: {e}"))?;
    let conn = RecoverableConnection::connect(Config {
        uri: settings.rabbitmq.unwrap().uri,
        time_out: Duration::from_secs(20),
        retry_delay: Duration::from_secs(1),
    })
    .await?;

    Ok(())
}

async fn dispatch<'tr>(
    pool: &PgPool,
    plugins: Plugins,
    message_dispatch_id: Id,
    template_render: &'tr dyn TemplateRender<TemplateData>,
) -> Result<(), anyhow::Error> {
    Ok(())

    //message_dispatch.commit(success, result)
}

#[derive(Debug)]
pub struct DefaultDispatchTemplate<'tr> {
    template: Template,
    render: &'tr dyn TemplateRender<TemplateData>,
}

impl<'tr> DefaultDispatchTemplate<'tr> {
    pub fn new(template: Template, render: &'tr dyn TemplateRender<TemplateData>) -> Self {
        Self { template, render }
    }
}
