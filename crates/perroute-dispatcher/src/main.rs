use anyhow::bail;
use perroute_commons::{
    configuration::settings::Settings,
    tracing::init_tracing,
    types::{
        id::Id,
        payload::Payload,
        recipient::Recipient,
        template::{TemplateData, TemplateError, TemplateRender},
        vars::Vars,
    },
};
use perroute_connectors::{
    plugin::{DispatchRequest, DispatchTemplate},
    Plugins,
};
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
use std::{ops::Deref, time::Duration};
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
    template_render: &'tr dyn TemplateRender,
) -> Result<(), anyhow::Error> {
    let message_dispatch = fetch_message_dispatch(pool, message_dispatch_id).await?;
    if *message_dispatch.status() != MessageDispatchStatus::Pending {
        bail!("Message dispatch is not pending");
    }
    let message = message_dispatch.message(pool).await?;
    let channel = message.channel(pool).await?;
    let message_type = message.message_type(pool).await?;
    let schema = message.schema(pool).await?;
    let route = message_dispatch.route(pool).await?;
    let connection = route.connection(pool).await?;
    let connector_plugin = plugins.get(connection.plugin_id()).unwrap();
    let dispatcher = connector_plugin.dispatcher(*route.dispatch_type()).unwrap();
    let template = route.template(pool).await?;

    let vars = channel
        .vars()
        .merge(message_type.vars())
        .merge(schema.vars())
        .merge(
            template
                .as_ref()
                .map(|t| t.vars())
                .unwrap_or(&Vars::default()),
        );

    let template = template.map(|t| DefaultDispatchTemplate::new(t, template_render));
    let template = template.as_ref().map(|t| t as &dyn DispatchTemplate);

    let req = DispatchRequest {
        id: message_dispatch_id,
        connection_properties: connection.properties(),
        dispatch_properties: route.dispatcher_properties(),
        template,
        recipient: message.recipient().as_ref(),
        payload: message.payload(),
        vars: &vars,
    };

    let result = dispatcher.dispatch(&req);

    Ok(())

    //message_dispatch.commit(success, result)
}

#[derive(Debug)]
pub struct DefaultDispatchTemplate<'tr> {
    template: Template,
    render: &'tr dyn TemplateRender,
}

impl<'tr> DefaultDispatchTemplate<'tr> {
    pub fn new(template: Template, render: &'tr dyn TemplateRender) -> Self {
        Self { template, render }
    }
}

impl<'tr> DispatchTemplate for DefaultDispatchTemplate<'tr> {
    fn render_subject(&self, data: &TemplateData) -> Result<Option<String>, TemplateError> {
        self.template
            .subject()
            .as_ref()
            .map(|s| self.render.render(s.as_ref(), data))
            .transpose()
    }

    fn render_text(&self, data: &TemplateData) -> Result<Option<String>, TemplateError> {
        self.template
            .text()
            .as_ref()
            .map(|s| self.render.render(s.as_ref(), data))
            .transpose()
    }

    fn render_html(&self, data: &TemplateData) -> Result<Option<String>, TemplateError> {
        self.template
            .html()
            .as_ref()
            .map(|s| self.render.render(s.as_ref(), data))
            .transpose()
    }
}

fn template_data(payload: &Payload, recipient: &Recipient, vars: &Vars) -> TemplateData {
    TemplateData {
        payload: payload.clone(),
        recipient: recipient.clone(),
        vars: vars.clone(),
    }
}

async fn fetch_message_dispatch(pool: &PgPool, id: Id) -> Result<MessageDispatch, anyhow::Error> {
    Ok(MessageDispatch::find(
        pool,
        MessageDispatchQueryBuilder::default()
            .id(Some(id))
            .build()
            .unwrap(),
    )
    .await
    .unwrap()
    .unwrap())
}
