use crate::{
    command,
    command_bus::{
        bus::CommandBusContext, commands::CommandType, error::CommandBusError,
        handlers::CommandHandler,
    },
    into_event,
};
use async_trait::async_trait;
use perroute_commons::types::{actor::Actor, id::Id};
use perroute_storage::{
    models::business_unit::{BusinessUnit, BusinessUnitQueryBuilder},
    query::FetchableModel,
};

command!(
    DeleteBusinessUnitCommand,
    CommandType::DeleteBusinessUnit,
    business_unit_id: Id
);
into_event!(DeleteBusinessUnitCommand);

#[derive(Debug)]
pub struct DeleteBusinessUnitCommandHandler;

#[derive(thiserror::Error, Debug, Clone)]
pub enum DeleteBusinessUnitError {
    #[error("BusinessUnit with id {0} nor found")]
    BusinessUnitNotFound(Id),
}

#[async_trait]
impl CommandHandler for DeleteBusinessUnitCommandHandler {
    type Command = DeleteBusinessUnitCommand;
    type Output = bool;

    #[tracing::instrument(name = "delete_business_unit_handler", skip(self, ctx))]
    async fn handle<'ctx>(
        &self,
        ctx: &mut CommandBusContext<'ctx>,
        actor: &Actor,
        command: Self::Command,
    ) -> Result<bool, CommandBusError> {
        let bu = BusinessUnit::find(
            ctx.tx(),
            BusinessUnitQueryBuilder::default()
                .id(Some(*command.business_unit_id()))
                .build()
                .unwrap(),
        )
        .await?;

        Ok(true)
    }
}
