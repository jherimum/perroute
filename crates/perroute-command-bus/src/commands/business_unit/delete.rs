use crate::{
    bus::{CommandBusContext, CommandHandler, CommandHandlerOutput, CommandHandlerResult},
    commands::Command,
};
use bon::Builder;
use perroute_commons::{
    events::{BusinessUnitDeletedEvent, Event, EventData, EventType},
    types::id::Id,
};
use perroute_storage::repository::{
    business_units::{BusinessUnitQuery, BusinessUnitRepository},
    TransactedRepository,
};
use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum DeleteBusinessUnitCommandError {
    #[error("Business unit not found")]
    BusinessUnitNotFound,
}

#[derive(Debug, Clone, Builder, Serialize)]
pub struct DeleteBusinessUnitCommand {
    pub id: Id,
}

impl Command for DeleteBusinessUnitCommand {
    fn event_type(&self) -> perroute_commons::events::EventType {
        perroute_commons::events::EventType::BusinessUnitDeleted
    }

    fn entity_id(&self) -> &Id {
        &self.id
    }
}

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
            &BusinessUnitQuery::ById(cmd.inner().id.clone()),
        )
        .await?;

        if !exists {
            return Err(DeleteBusinessUnitCommandError::BusinessUnitNotFound.into());
        }

        BusinessUnitRepository::delete_business_unit(ctx.repository(), &cmd.inner().id).await?;

        Ok(CommandHandlerOutput::new(
            (),
            BusinessUnitDeletedEvent::builder()
                .id(cmd.inner().id.clone())
                .build(),
        ))
    }
}
