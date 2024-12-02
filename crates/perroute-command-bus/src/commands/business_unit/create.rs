use crate::{
    bus::{CommandBusContext, CommandHandler, CommandHandlerResult},
    commands::Command,
};
use bon::{builder, Builder};
use perroute_commons::types::{code::Code, id::Id, name::Name, vars::Vars};
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
    business_unit_id: Id,
    name: Name,
    code: Code,
    vars: Vars,
}

impl Command for CreateBusinessUnitCommand {
    fn entity_id(&self) -> &Id {
        &self.business_unit_id
    }

    fn event_type(&self) -> perroute_commons::events::EventType {
        perroute_commons::events::EventType::BusinessUnitCreated
    }
}

pub struct CreateBusinessUnitCommandHandler;

impl CommandHandler for CreateBusinessUnitCommandHandler {
    type Command = CreateBusinessUnitCommand;
    type Output = BusinessUnit;

    async fn handle<R: TransactedRepository>(
        &self,
        cmd: &crate::commands::CommandWrapper<'_, Self::Command>,
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
