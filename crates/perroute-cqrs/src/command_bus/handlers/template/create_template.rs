use crate::{
    command,
    command_bus::{
        bus::CommandBusContext, commands::CommandType, handlers::CommandHandler, Result,
    },
    into_event,
};
use anyhow::Context;
use async_trait::async_trait;
use derive_getters::Getters;
use perroute_commons::types::{id::Id, template::TemplateSnippet};
use perroute_storage::models::template::{Template, TemplateBuilder};
use sqlx::PgPool;
use tap::TapFallible;

command!(
    CreateTemplateCommand,
    CommandType::CreateTemplate,
    id: Id,
    name: String,
    subject: Option<TemplateSnippet>,
    html: Option<TemplateSnippet>,
    text: Option<TemplateSnippet>
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
pub enum CreateTemplateError {}

#[async_trait]
impl CommandHandler for CreateTemplateCommandHandler {
    type Command = CreateTemplateCommand;
    type Output = Template;

    #[tracing::instrument(name = "create_template_handler", skip(self, ctx))]
    async fn handle<'tx>(
        &self,
        ctx: &mut CommandBusContext,

        cmd: Self::Command,
    ) -> Result<Self::Output> {
        Ok(TemplateBuilder::default()
            .id(cmd.id)
            .name(cmd.name)
            .subject(cmd.subject)
            .text(cmd.text)
            .html(cmd.html)
            .build()
            .context("Failed to build template")?
            .save(ctx.pool())
            .await
            .tap_err(|e| tracing::error!("Failed to save template:{e}"))?)
    }
}
