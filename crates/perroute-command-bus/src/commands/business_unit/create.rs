use crate::bus::{
    Command, CommandBusContext, CommandHandler, CommandHandlerOutput, CommandHandlerResult,
};
use bon::Builder;
use perroute_commons::{
    commands::CommandType,
    events::Event,
    types::{code::Code, id::Id, name::Name, vars::Vars, Timestamp},
};
use perroute_storage::{
    models::business_unit::BusinessUnit,
    repository::{
        business_units::{BusinessUnitQuery, BusinessUnitRepository},
        TransactedRepository,
    },
};
use serde::Serialize;
use tap::TapFallible;

#[derive(Debug, thiserror::Error)]
pub enum CreateBusinessUnitCommandError {
    #[error("Business unit with code {0} already exists")]
    CodeAlreadyExists(Code),
}

#[derive(Debug, Clone, Builder, Serialize)]
pub struct CreateBusinessUnitCommand {
    name: Name,
    code: Code,
    vars: Option<Vars>,
}

impl Command for CreateBusinessUnitCommand {
    fn command_type(&self) -> CommandType {
        CommandType::CreateBusinessUnit
    }
}

pub struct CreateBusinessUnitCommandHandler;

impl CommandHandler for CreateBusinessUnitCommandHandler {
    type Command = CreateBusinessUnitCommand;
    type Output = BusinessUnit;

    async fn handle<R: TransactedRepository>(
        &self,
        cmd: &Self::Command,
        ctx: CommandBusContext<'_, R>,
    ) -> CommandHandlerResult<Self::Output> {
        let exists = BusinessUnitRepository::exists_business_unit(
            ctx.repository(),
            &BusinessUnitQuery::ByCode(cmd.code.clone()),
        )
        .await
        .tap_err(|e| log::error!("Error checking if business unit exists: {:?}", e))?;

        if exists {
            return Err(CreateBusinessUnitCommandError::CodeAlreadyExists(cmd.code.clone()).into());
        }

        let bu = BusinessUnit::builder()
            .id(Id::default())
            .code(cmd.code.clone())
            .name(cmd.name.clone())
            .maybe_vars(cmd.vars.clone())
            .created_at(Timestamp::now())
            .updated_at(Timestamp::now())
            .build();

        let bu = BusinessUnitRepository::save_business_unit(ctx.repository(), bu)
            .await
            .tap_err(|e| log::error!("Error saving business unit: {:?}", e))?;

        CommandHandlerOutput::new(bu.clone())
            .with_event(Event::BusinessUnitCreated(bu.id().clone()))
            .ok()
    }
}
