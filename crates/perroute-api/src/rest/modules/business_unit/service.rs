use super::models::BusinessUnitCollectionPath;
use crate::rest::{
    error::ApiError,
    models::{ResourceModel, ResourceModelCollection},
    modules::business_unit::models::{
        BusinessUnitModel, BusinessUnitPath, CreateBusinessUnitRequest, UpdateBusinessUnitRequest,
    },
    service::RestService,
    ResourceModelCollectionResult, ResourceModelResult, RestServiceResult,
};
use perroute_command_bus::{
    commands::business_unit::{
        create::{CreateBusinessUnitCommand, CreateBusinessUnitCommandHandler},
        delete::{DeleteBusinessUnitCommand, DeleteBusinessUnitCommandHandler},
        update::{UpdateBusinessUnitCommand, UpdateBusinessUnitCommandHandler},
    },
    CommandBus, CommandBusError,
};
use perroute_commons::types::actor::Actor;
use perroute_query_bus::{queries::business_unit::QueryBusinessUnitsHandler, QueryBus};
use perroute_storage::{
    models::business_unit::BusinessUnit, repository::business_units::BusinessUnitQuery,
};
use std::future::Future;

pub trait BusinessUnitRestService {
    fn get(
        &self,
        actor: &Actor,
        path: &BusinessUnitPath,
    ) -> impl Future<Output = ResourceModelResult<BusinessUnitModel>>;

    fn query(
        &self,
        actor: &Actor,
        path: &BusinessUnitCollectionPath,
    ) -> impl Future<Output = ResourceModelCollectionResult<BusinessUnitModel>>;

    fn delete(
        &self,
        actor: &Actor,
        path: &BusinessUnitPath,
    ) -> impl Future<Output = RestServiceResult<bool>>;

    fn update(
        &self,
        actor: &Actor,
        path: &BusinessUnitPath,
        payload: &UpdateBusinessUnitRequest,
    ) -> impl Future<Output = ResourceModelResult<BusinessUnitModel>>;

    fn create(
        &self,
        actor: &Actor,
        path: &BusinessUnitCollectionPath,
        payload: &CreateBusinessUnitRequest,
    ) -> impl Future<Output = ResourceModelResult<BusinessUnitModel>>;
}

async fn query<QB: QueryBus>(
    query_bus: &QB,
    query: &BusinessUnitQuery,
) -> RestServiceResult<Vec<BusinessUnit>> {
    query_bus
        .execute::<_, QueryBusinessUnitsHandler, _>(&Actor::System, query)
        .await
        .map_err(|e| ApiError::InternalServerError(e.into()))
}

impl<CB: CommandBus, QB: QueryBus> BusinessUnitRestService for RestService<CB, QB> {
    async fn get(
        &self,
        actor: &Actor,
        path: &BusinessUnitPath,
    ) -> ResourceModelResult<BusinessUnitModel> {
        let query_result = query(self.query_bus(), &BusinessUnitQuery::ById(path.id())).await?;
        let bu = query_result.first().ok_or_else(|| ApiError::NotFound)?;
        Ok(ResourceModel::new(BusinessUnitModel::from(bu)))
    }

    async fn query(
        &self,
        actor: &Actor,
        path: &BusinessUnitCollectionPath,
    ) -> ResourceModelCollectionResult<BusinessUnitModel> {
        let query_result = query(self.query_bus(), &BusinessUnitQuery::All).await?;

        Ok(ResourceModelCollection {
            data: query_result
                .iter()
                .map(BusinessUnitModel::from)
                .map(ResourceModel::new)
                .collect::<Vec<_>>(),
        })
    }

    async fn delete(&self, actor: &Actor, path: &BusinessUnitPath) -> RestServiceResult<bool> {
        Ok(self
            .command_bus()
            .execute::<_, DeleteBusinessUnitCommandHandler, _, _>(
                actor,
                &DeleteBusinessUnitCommand::builder().id(path.id()).build(),
            )
            .await?)
    }

    async fn update(
        &self,
        actor: &Actor,
        id: &BusinessUnitPath,
        payload: &UpdateBusinessUnitRequest,
    ) -> ResourceModelResult<BusinessUnitModel> {
        let cmd = UpdateBusinessUnitCommand::builder()
            .id(id.id())
            .name(payload.name()?)
            .maybe_vars(payload.vars()?)
            .build();

        let bu = self
            .command_bus()
            .execute::<_, UpdateBusinessUnitCommandHandler, _, _>(actor, &cmd)
            .await
            .map_err(CommandBusError::from)?;

        Ok(ResourceModel::new(BusinessUnitModel::from(&bu)))
    }

    async fn create(
        &self,
        actor: &Actor,
        _path: &BusinessUnitCollectionPath,
        payload: &CreateBusinessUnitRequest,
    ) -> ResourceModelResult<BusinessUnitModel> {
        let cmd = CreateBusinessUnitCommand::builder()
            .name(payload.name()?)
            .code(payload.code()?)
            .maybe_vars(payload.vars()?)
            .build();

        Ok(self
            .command_bus()
            .execute::<_, CreateBusinessUnitCommandHandler, _, _>(actor, &cmd)
            .await
            .map(|bu| BusinessUnitModel::from(&bu))
            .map(ResourceModel::new)?)
    }
}
