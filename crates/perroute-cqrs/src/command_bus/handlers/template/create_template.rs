use crate::{
    command,
    command_bus::{
        bus::CommandBusContext, commands::CommandType, handlers::CommandHandler, Result,
    },
    into_event,
};
use anyhow::Context;
use async_trait::async_trait;
use chrono::NaiveDateTime;
use derive_getters::Getters;
use perroute_commons::types::{
    actor::Actor, id::Id, priority::Priority, template::TemplateSnippet, vars::Vars,
};
use perroute_connectors::types::dispatch_type::DispatchType;
use perroute_storage::{
    models::{
        schema::{Schema, SchemasQuery},
        template::{Template, TemplateBuilder},
    },
    query::FetchableModel,
};
use sqlx::PgPool;
use tap::TapFallible;

command!(
    CreateTemplateCommand,
    CommandType::CreateTemplate,
    id: Id,
    name: String,
    subject: Option<TemplateSnippet>,
    html: Option<TemplateSnippet>,
    text: Option<TemplateSnippet>,
    dispatch_type: DispatchType,
    schema_id: Id,
    vars: Vars,
    start_at: NaiveDateTime,
    end_at: Option<NaiveDateTime>,
    priority: Priority
);
into_event!(CreateTemplateCommand);

#[derive(Debug, Getters)]
pub struct CreateTemplateCommandHandler {
    pool: PgPool,
}

impl CreateTemplateCommandHandler {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[derive(thiserror::Error, Debug, Clone)]
pub enum CreateTemplateError {
    #[error("Schema not found: {0}")]
    SchemaNotFound(Id),
}

#[async_trait]
impl CommandHandler for CreateTemplateCommandHandler {
    type Command = CreateTemplateCommand;
    type Output = Template;

    #[tracing::instrument(name = "create_template_handler", skip(self, ctx))]
    async fn handle<'tx>(
        &self,
        ctx: &mut CommandBusContext<'tx>,
        _: &Actor,
        cmd: Self::Command,
    ) -> Result<Self::Output> {
        let schema = Schema::find(ctx.pool(), SchemasQuery::with_id(cmd.schema_id))
            .await
            .tap_err(|e| tracing::error!("Failed to retrieve schema: {e}"))?
            .ok_or(CreateTemplateError::SchemaNotFound(cmd.schema_id))?;

        Ok(TemplateBuilder::default()
            .schema_id(cmd.schema_id)
            .id(cmd.id)
            .name(cmd.name)
            .subject(cmd.subject)
            .text(cmd.text)
            .html(cmd.html)
            .active(false)
            .business_unit_id(*schema.business_unit_id())
            .dispatch_type(cmd.dispatch_type)
            .message_type_id(*schema.message_type_id())
            .vars(cmd.vars)
            .start_at(cmd.start_at)
            .end_at(cmd.end_at)
            .priority(cmd.priority)
            .build()
            .context("Failed to build template")?
            .save(ctx.pool())
            .await
            .tap_err(|e| tracing::error!("Failed to save template:{e}"))?)
    }
}
