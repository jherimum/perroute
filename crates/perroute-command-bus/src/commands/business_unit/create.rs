use crate::{
    bus::{CommandBusContext, CommandHandler, CommandHandlerResult},
    commands::Command,
    impl_command, CommandBusError,
};
use perroute_commons::{
    events::BusinessUnitCreatedEvent,
    types::{code::Code, name::Name, vars::Vars},
};
use perroute_storage::{
    active_record::{
        business_unit::{BusinessUnitQuery, CreateBusinessUnit},
        datasource::Connection,
        ActiveRecord,
    },
    models::business_unit::BusinessUnit,
};
use tap::TapFallible;

#[derive(Debug, thiserror::Error)]
pub enum CreateBusinessUnitCommandError {
    #[error("Business unit with code {0} already exists")]
    CodeAlreadyExists(Code),
}

impl_command!(CreateBusinessUnitCommand, {
    name: Name,
    code: Code,
    vars: Vars,
});

pub struct CreateBusinessUnitCommandHandler;

async fn exists_code<C: AsRef<Connection>>(
    datasource: &C,
    code: &Code,
) -> Result<bool, CommandBusError> {
    BusinessUnit::exists(datasource, BusinessUnitQuery::ByCode(code))
        .await
        .map_err(CommandBusError::from)
}

impl CommandHandler for CreateBusinessUnitCommandHandler {
    type Command = CreateBusinessUnitCommand;
    type Output = BusinessUnit;
    type E = BusinessUnitCreatedEvent;

    fn into_event(
        command: &Self::Command,
        output: &Self::Output,
    ) -> Option<BusinessUnitCreatedEvent> {
        Some(
            BusinessUnitCreatedEvent::builder()
                .business_unit_id(output.id())
                .code(&command.code)
                .name(&command.name)
                .vars(&command.vars)
                .build(),
        )
    }

    async fn handle<C: AsRef<Connection>>(
        &self,
        ctx: &CommandBusContext<'_, C, CreateBusinessUnitCommand>,
    ) -> CommandHandlerResult<Self::Output> {
        if exists_code(ctx.datasource(), &ctx.command().code).await? {
            return Err(CreateBusinessUnitCommandError::CodeAlreadyExists(
                ctx.command().code.clone(),
            )
            .into());
        }

        BusinessUnit::create(ctx.datasource(), ctx.into())
            .await
            .tap_err(|e| log::error!("Error creating business unit: {:?}", e))
            .map_err(CommandBusError::from)
    }
}

impl<C: AsRef<Connection>>
    From<&CommandBusContext<'_, C, CreateBusinessUnitCommand>>
    for CreateBusinessUnit
{
    fn from(ctx: &CommandBusContext<'_, C, CreateBusinessUnitCommand>) -> Self {
        CreateBusinessUnit::builder()
            .code(&ctx.command().code)
            .name(&ctx.command().name)
            .vars(&ctx.command().vars)
            .timestamp(ctx.timestamp())
            .build()
    }
}
