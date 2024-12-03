use crate::{
    bus::{CommandBusContext, CommandHandler, CommandHandlerOutput, CommandHandlerResult},
    commands::Command,
    impl_command,
};
use bon::{builder, Builder};
use perroute_commons::{
    events::BusinessUnitCreatedEvent,
    types::{code::Code, id::Id, name::Name, vars::Vars},
};
use perroute_storage::{
    models::business_unit::BusinessUnit,
    repository::{
        business_units::{BusinessUnitQuery, BusinessUnitRepository},
        TransactedRepository,
    },
};
use tap::TapFallible;

#[derive(Debug, thiserror::Error)]
pub enum CreateBusinessUnitCommandError {
    #[error("Business unit with code {0} already exists")]
    CodeAlreadyExists(Code),
}

impl_command!(CreateBusinessUnitCommand, {
    business_unit_id: Id,
    name: Name,
    code: Code,
    vars: Vars,
});

pub struct CreateBusinessUnitCommandHandler;

impl CommandHandler for CreateBusinessUnitCommandHandler {
    type Command = CreateBusinessUnitCommand;
    type Output = BusinessUnit;
    type ApplicationEvent = BusinessUnitCreatedEvent;

    async fn handle<R: TransactedRepository>(
        &self,
        cmd: &crate::commands::CommandWrapper<'_, Self::Command>,
        ctx: &CommandBusContext<'_, R>,
    ) -> CommandHandlerResult<Self::Output, Self::ApplicationEvent> {
        let exists = BusinessUnitRepository::exists_business_unit(
            ctx.repository(),
            &BusinessUnitQuery::ByCode(cmd.inner().code.clone()),
        )
        .await
        .tap_err(|e| log::error!("Error checking if business unit exists: {:?}", e))?;

        if exists {
            return Err(CreateBusinessUnitCommandError::CodeAlreadyExists(
                cmd.inner().code.clone(),
            )
            .into());
        }

        let bu = BusinessUnit::builder()
            .id(Id::new())
            .code(cmd.inner().code.clone())
            .name(cmd.inner().name.clone())
            .vars(cmd.inner().vars.clone())
            .created_at(cmd.created_at().clone())
            .updated_at(cmd.created_at().clone())
            .build();

        let bu = BusinessUnitRepository::save_business_unit(ctx.repository(), bu)
            .await
            .tap_err(|e| log::error!("Error saving business unit: {:?}", e))?;

        Ok(CommandHandlerOutput::new(
            bu,
            BusinessUnitCreatedEvent::builder()
                .business_unit_id(cmd.command.business_unit_id.clone())
                .code(cmd.command.code.clone())
                .name(cmd.command.name.clone())
                .vars(cmd.command.vars.clone())
                .build(),
        ))
    }
}
