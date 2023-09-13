use crate::{
    command_bus::{
        bus::CommandBusContext, commands::CommandType, handlers::CommandHandler, Result,
    },
    impl_command, into_event,
};
use anyhow::Context;
use async_trait::async_trait;
use derive_builder::Builder;
use derive_getters::Getters;
use perroute_commons::types::{actor::Actor, code::Code, id::Id, vars::Vars};
use perroute_storage::{
    models::business_unit::{BusinessUnit, BusinessUnitBuilder, BusinessUnitQuery},
    query::FetchableModel,
};
use serde::Serialize;
use sqlx::PgPool;
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
impl_command!(CreateBusinessUnitCommand, CommandType::CreateBusinessUnit);
into_event!(CreateBusinessUnitCommand);

#[derive(Debug)]
pub struct CreateBusinessUnitCommandHandler {
    pool: PgPool,
}

impl CreateBusinessUnitCommandHandler {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

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
    ) -> Result<Self::Output> {
        let code_exists =
            BusinessUnit::exists(&self.pool, BusinessUnitQuery::with_code(cmd.code.clone()))
                .await
                .tap_err(|e| {
                    tracing::error!(
                        "Failed to check if business Unit with code {} exists: {e}",
                        cmd.code
                    );
                })?;

        if code_exists {
            return Err(CreateBusinessUnitError::CodeAlreadyExists(cmd.code).into());
        }

        Ok(BusinessUnitBuilder::default()
            .id(cmd.id)
            .code(cmd.code)
            .name(cmd.name)
            .vars(cmd.vars)
            .build()
            .context("Failed to build BusinessUnit")?
            .save(&self.pool)
            .await
            .tap_err(|e| tracing::error!("Failed to save BusinessUnit: {e}"))?)
    }
}
