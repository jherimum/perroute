use crate::bus::{
    Command, CommandBusContext, CommandHandler, CommandHandlerOutput, CommandHandlerResult,
};
use bon::Builder;
use perroute_commons::{
    commands::CommandType,
    events::{Event, EventData, EventType},
    types::{actor::Actor, code::Code, id::Id, name::Name, vars::Vars, Timestamp},
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
    #[builder(default)]
    id: Id,
    name: Name,
    code: Code,
    vars: Vars,
    #[builder(default)]
    created_at: Timestamp,
}

impl Command for CreateBusinessUnitCommand {
    fn command_type(&self) -> CommandType {
        CommandType::CreateBusinessUnit
    }

    fn to_event(&self, actor: &Actor) -> Event {
        Event::BusinessUnitCreated(
            EventData::builder()
                .actor(actor.clone())
                .created_at(self.created_at.clone())
                .entity_id(self.id.clone())
                .payload(())
                .event_type(EventType::BusinessUnitCreated)
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
            .id(cmd.id.clone())
            .code(cmd.code.clone())
            .name(cmd.name.clone())
            .vars(cmd.vars.clone())
            .created_at(Timestamp::now())
            .updated_at(Timestamp::now())
            .build();

        let bu = BusinessUnitRepository::save_business_unit(ctx.repository(), bu)
            .await
            .tap_err(|e| log::error!("Error saving business unit: {:?}", e))?;

        CommandHandlerOutput::new(bu.clone()).ok()
    }
}
