use crate::{bus::Ctx, command::Command, error::CommandBusError};
use anyhow::Context;
use async_trait::async_trait;
use perroute_commons::types::{
    actor::Actor, command_type::CommandType, id::Id, template::TemplateSnippet,
};
use perroute_storage::models::template::{Template, TemplateBuilder};
use tap::TapFallible;

#[derive(Debug, derive_builder::Builder)]
pub struct CreateTemplateCommand {
    id: Id,
    name: String,
    subject: Option<TemplateSnippet>,
    html: Option<TemplateSnippet>,
    text: Option<TemplateSnippet>,
}

#[derive(thiserror::Error, Debug, Clone)]
pub enum CreateTemplateError {}

#[async_trait]
impl Command for CreateTemplateCommand {
    type Output = Template;

    #[tracing::instrument(name = "create_template_handler", skip(self, ctx))]
    async fn handle<'tx>(&self, ctx: &mut Ctx<'tx>) -> Result<Self::Output, CommandBusError> {
        Ok(TemplateBuilder::default()
            .id(self.id)
            .name(self.name.clone())
            .subject(self.subject.clone())
            .text(self.text.clone())
            .html(self.html.clone())
            .build()
            .context("Failed to build template")?
            .save(ctx.pool())
            .await
            .tap_err(|e| tracing::error!("Failed to save template:{e}"))?)
    }

    fn command_type(&self) -> CommandType {
        CommandType::CreateTemplate
    }

    fn supports(&self, actor: &Actor) -> bool {
        true
    }
}
