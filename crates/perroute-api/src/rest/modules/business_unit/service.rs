use super::controller::{BusinessUnitCollectionPath, BusinessUnitPath};
use crate::rest::{
    error::ApiError,
    models::resource::{ResourceModel, ResourceModelCollection},
    modules::business_unit::models::{
        BusinessUnitModel, CreateBusinessUnitRequest, UpdateBusinessUnitRequest,
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

    fn try_get(
        &self,
        actor: &Actor,
        path: &BusinessUnitPath,
    ) -> impl Future<Output = Result<Option<ResourceModel<BusinessUnitModel>>, ApiError>>;

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
        Ok(self
            .try_get(actor, path)
            .await?
            .ok_or_else(|| ApiError::NotFound)?)
    }

    async fn try_get(
        &self,
        actor: &Actor,
        path: &BusinessUnitPath,
    ) -> Result<Option<ResourceModel<BusinessUnitModel>>, ApiError> {
        let query_result = query(self.query_bus(), &BusinessUnitQuery::ById(path.id())).await?;
        Ok(query_result.first().map(|bu| bu.clone().into()))
    }

    async fn query(
        &self,
        actor: &Actor,
        path: &BusinessUnitCollectionPath,
    ) -> ResourceModelCollectionResult<BusinessUnitModel> {
        let query_result = query(self.query_bus(), &BusinessUnitQuery::All).await?;

        Ok(ResourceModelCollection::new(
            query_result
                .into_iter()
                .map(BusinessUnitModel::from)
                .map(ResourceModel::new)
                .collect::<Vec<_>>(),
        ))
    }

    async fn delete(&self, actor: &Actor, path: &BusinessUnitPath) -> RestServiceResult<bool> {
        Ok(self
            .command_bus()
            .execute::<_, DeleteBusinessUnitCommandHandler, _>(
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
            .execute::<_, UpdateBusinessUnitCommandHandler, _>(actor, &cmd)
            .await
            .map_err(CommandBusError::from)?;

        Ok(bu.into())
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
            .execute::<_, CreateBusinessUnitCommandHandler, _>(actor, &cmd)
            .await
            .map(|bu| bu.into())?)
    }
}
