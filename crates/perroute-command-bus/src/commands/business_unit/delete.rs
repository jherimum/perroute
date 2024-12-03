use crate::{
    bus::{CommandBusContext, CommandHandler, CommandHandlerOutput, CommandHandlerResult},
    commands::Command,
    impl_command,
};
use perroute_commons::{events::BusinessUnitDeletedEvent, types::id::Id};
use perroute_storage::repository::{
    business_units::{BusinessUnitQuery, BusinessUnitRepository},
    TransactedRepository,
};

#[derive(Debug, thiserror::Error)]
pub enum DeleteBusinessUnitCommandError {
    #[error("Business unit not found")]
    BusinessUnitNotFound,
}

impl_command!(DeleteBusinessUnitCommand, {
    business_unit_id: Id
});

pub struct DeleteBusinessUnitCommandHandler;

impl CommandHandler for DeleteBusinessUnitCommandHandler {
    type Command = DeleteBusinessUnitCommand;
    type Output = ();
    type ApplicationEvent = BusinessUnitDeletedEvent;

    async fn handle<R: TransactedRepository>(
        &self,
        cmd: &crate::commands::CommandWrapper<'_, Self::Command>,
        ctx: &CommandBusContext<'_, R>,
    ) -> CommandHandlerResult<Self::Output, Self::ApplicationEvent> {
        let exists = BusinessUnitRepository::exists_business_unit(
            ctx.repository(),
            &BusinessUnitQuery::ById(cmd.inner().business_unit_id.clone()),
        )
        .await?;

        if !exists {
            return Err(DeleteBusinessUnitCommandError::BusinessUnitNotFound.into());
        }

        BusinessUnitRepository::delete_business_unit(
            ctx.repository(),
            &cmd.inner().business_unit_id,
        )
        .await?;

        Ok(CommandHandlerOutput::new(
            (),
            BusinessUnitDeletedEvent::builder()
                .business_unit_id(cmd.inner().business_unit_id.clone())
                .build(),
        ))
    }
}
