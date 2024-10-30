use crate::{
    bus::{Command, CommandBusContext, CommandHandler},
    CommandBusResult,
};
use bon::Builder;
use perroute_commons::types::{id::Id, vars::Vars, Code, Name, Timestamp};
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

#[derive(Debug, Clone, Builder)]
pub struct CreateBusinessUnitCommand {
    name: Name,
    code: Code,
    vars: Option<Vars>,
}

impl Command for CreateBusinessUnitCommand {}

pub struct CreateBusinessUnitCommandHandler;

impl CommandHandler for CreateBusinessUnitCommandHandler {
    type Command = CreateBusinessUnitCommand;

    type Output = BusinessUnit;

    async fn handle<R: TransactedRepository>(
        &self,
        cmd: &Self::Command,
        ctx: CommandBusContext<'_, R>,
    ) -> CommandBusResult<Self::Output> {
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

        Ok(
            BusinessUnitRepository::save_business_unit(ctx.repository(), bu)
                .await
                .tap_err(|e| log::error!("Error saving business unit: {:?}", e))?,
        )
    }
}
