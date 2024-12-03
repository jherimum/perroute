use crate::{
    bus::{CommandBusContext, CommandHandler, CommandHandlerOutput, CommandHandlerResult},
    commands::Command,
    CommandBusError,
};
use bon::Builder;
use perroute_commons::{
    events::BusinessUnitUpdatedEvent,
    types::{id::Id, name::Name, vars::Vars},
};
use perroute_storage::{
    models::business_unit::BusinessUnit,
    repository::{
        business_units::{BusinessUnitQuery, BusinessUnitRepository},
        TransactedRepository,
    },
};
use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum UpdateBusinessUnitCommandError {
    #[error("Business unit not found")]
    NotFound,
}

#[derive(Debug, Clone, Builder, Serialize)]
pub struct UpdateBusinessUnitCommand {
    pub id: Id,
    pub name: Name,
    pub vars: Vars,
}

impl Command for UpdateBusinessUnitCommand {
    fn event_type(&self) -> perroute_commons::events::EventType {
        perroute_commons::events::EventType::BusinessUnitUpdated
    }

    fn entity_id(&self) -> &Id {
        &self.id
    }
}

pub struct UpdateBusinessUnitCommandHandler;

impl CommandHandler for UpdateBusinessUnitCommandHandler {
    type Command = UpdateBusinessUnitCommand;
    type Output = BusinessUnit;
    type ApplicationEvent = BusinessUnitUpdatedEvent;

    async fn handle<R: TransactedRepository>(
        &self,
        cmd: &crate::commands::CommandWrapper<'_, Self::Command>,
        ctx: &CommandBusContext<'_, R>,
    ) -> CommandHandlerResult<Self::Output, Self::ApplicationEvent> {
        let bu = match BusinessUnitRepository::find_business_unit(
            ctx.repository(),
            &BusinessUnitQuery::ById(cmd.inner().id.clone()),
        )
        .await?
        {
            Some(bu) => {
                let bu = BusinessUnitRepository::update_business_unit(
                    ctx.repository(),
                    bu.set_name(cmd.inner().name.clone())
                        .set_vars(cmd.inner().vars.clone())
                        .set_updated_at(cmd.created_at().clone()),
                )
                .await?;
                Ok(bu)
            }
            None => Err(CommandBusError::from(
                UpdateBusinessUnitCommandError::NotFound,
            )),
        }?;

        Ok(CommandHandlerOutput::new(
            bu,
            BusinessUnitUpdatedEvent::builder()
                .id(cmd.command.id.clone())
                .name(cmd.command.name.clone())
                .vars(cmd.command.vars.clone())
                .build(),
        ))
    }
}
