use crate::bus::{
    Command, CommandBusContext, CommandHandler, CommandHandlerResult, CommandWrapper,
};
use bon::Builder;
use perroute_commons::{
    commands::CommandType,
    events::{Event, EventData, EventType},
    types::{code::Code, id::Id, name::Name, vars::Vars},
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
    vars: Vars,
}

impl Command for CreateBusinessUnitCommand {
    type Output = BusinessUnit;

    fn command_type(&self) -> CommandType {
        CommandType::CreateBusinessUnit
    }

    fn to_event(
        &self,
        created_at: &perroute_commons::types::Timestamp,
        actor: &perroute_commons::types::actor::Actor,
        output: &Self::Output,
    ) -> Event {
        Event::BusinessUnitCreated(
            EventData::builder()
                .id(Id::new())
                .entity_id(output.id().clone())
                .event_type(EventType::BusinessUnitCreated)
                .actor(actor.clone())
                .payload(serde_json::to_value(self).unwrap())
                .created_at(created_at.clone())
                .build(),
        )
    }
}

pub struct CreateBusinessUnitCommandHandler;

impl CommandHandler for CreateBusinessUnitCommandHandler {
    type Command = CreateBusinessUnitCommand;
    type Output = BusinessUnit;

    async fn handle<R: TransactedRepository>(
        &self,
        cmd: CommandWrapper<'_, Self::Command>,
        ctx: &CommandBusContext<'_, R>,
    ) -> CommandHandlerResult<Self::Output> {
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

        Ok(bu)
    }
}
