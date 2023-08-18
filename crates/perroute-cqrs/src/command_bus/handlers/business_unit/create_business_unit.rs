use crate::{
    command_bus::{
        bus::CommandBusContext, commands::CommandType, error::CommandBusError,
        handlers::CommandHandler,
    },
    impl_command, into_event,
};
use anyhow::Context;
use async_trait::async_trait;
use derive_builder::Builder;
use derive_getters::Getters;
use perroute_commons::types::{actor::Actor, code::Code, id::Id, vars::Vars};
use perroute_messaging::events::EventType;
use perroute_storage::{
    models::business_unit::{BusinessUnit, BusinessUnitBuilder, BusinessUnitQuery},
    query::FetchableModel,
};
use serde::Serialize;
use tap::TapFallible;

#[derive(thiserror::Error, Debug, Clone)]
pub enum Error {
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
impl_command!(CreateBusinessUnitCommand, CommandType::CreateBusinessUnit);
into_event!(
    CreateBusinessUnitCommand,
    EventType::BusinessUnitCreated,
    |cmd: &CreateBusinessUnitCommand| { cmd.id }
);

#[derive(Debug)]
pub struct CreateBusinessUnitCommandHandler;

#[async_trait]
impl CommandHandler for CreateBusinessUnitCommandHandler {
    type Command = CreateBusinessUnitCommand;
    type Output = BusinessUnit;

    #[tracing::instrument(name = "create_business_unit_handler", skip(self, ctx))]
    async fn handle<'tx>(
        &self,
        ctx: &mut CommandBusContext<'tx>,
        _: &Actor,
        cmd: Self::Command,
    ) -> Result<Self::Output, CommandBusError> {
        let code_exists =
            BusinessUnit::exists(ctx.pool(), BusinessUnitQuery::with_code(cmd.code.clone()))
                .await
                .tap_err(|e| {
                    tracing::error!(
                        "Failed to check if business Unit with code {} exists:{e}",
                        cmd.code
                    );
                })?;

        if code_exists {
            return Err(Error::CodeAlreadyExists(cmd.code).into());
        }

        Ok(BusinessUnitBuilder::default()
            .id(cmd.id)
            .code(cmd.code)
            .name(cmd.name)
            .vars(cmd.vars)
            .build()
            .context("Failed to build BusinessUnit")?
            .save(ctx.tx())
            .await
            .tap_err(|e| tracing::error!("Failed to save BusinessUnit: {e}"))?)
    }
}
