use crate::{
    bus::Ctx,
    command::{Command, CommandResult},
};
use anyhow::Context;
use async_trait::async_trait;
use derive_builder::Builder;
use derive_getters::Getters;
use perroute_commons::types::{
    actor::Actor, code::Code, command_type::CommandType, id::Id, vars::Vars,
};
use perroute_storage::{
    models::business_unit::{BusinessUnit, BusinessUnitBuilder, BusinessUnitQuery},
    query::FetchableModel,
};
use serde::Serialize;
use tap::TapFallible;

#[derive(thiserror::Error, Debug, Clone)]
pub enum CreateBusinessUnitError {
    #[error("A BusinessUnit with code {0} already exists")]
    CodeAlreadyExists(Code),
}

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Builder, Getters)]
pub struct CreateBusinessUnitCommand {
    id: Id,
    code: Code,
    name: String,
    vars: Vars,
}

#[async_trait]
impl Command for CreateBusinessUnitCommand {
    type Output = BusinessUnit;

    #[tracing::instrument(name = "create_business_unit_handler", skip(self, ctx))]
    async fn handle<'ctx>(&self, ctx: &mut Ctx<'ctx>) -> CommandResult<Self::Output> {
        let code_exists =
            BusinessUnit::exists(ctx.pool(), BusinessUnitQuery::with_code(self.code.clone()))
                .await
                .tap_err(|e| {
                    tracing::error!(
                        "Failed to check if business Unit with code {} exists: {e}",
                        self.code
                    );
                })?;

        if code_exists {
            return Err(CreateBusinessUnitError::CodeAlreadyExists(self.code.clone()).into());
        }

        Ok(BusinessUnitBuilder::default()
            .id(self.id)
            .code(self.code.clone())
            .name(self.name.clone())
            .vars(self.vars.clone())
            .build()
            .context("Failed to build BusinessUnit")?
            .save(ctx.pool())
            .await
            .tap_err(|e| tracing::error!("Failed to save BusinessUnit: {e}"))?)
    }

    fn command_type(&self) -> CommandType {
        CommandType::CreateBusinessUnit
    }

    fn supports(&self, _actor: &Actor) -> bool {
        true
    }
}
