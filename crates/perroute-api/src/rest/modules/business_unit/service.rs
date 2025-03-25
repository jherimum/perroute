use crate::rest::{
    error::ApiError,
    models::resource::ResourceModel,
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
    CommandBus,
};
use perroute_commons::types::actor::Actor;
use perroute_query_bus::{QueryBus, QueryBusError};
use perroute_storage::{
    active_record::business_unit::BusinessUnitQuery,
    models::business_unit::BusinessUnit,
};
use std::future::Future;

use super::models::{BusinessUnitCollectionPath, BusinessUnitPath};

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
    ) -> impl Future<
        Output = Result<Option<ResourceModel<BusinessUnitModel>>, ApiError>,
    >;

    fn query(
        &self,
        actor: &Actor,
        path: &BusinessUnitCollectionPath,
    ) -> impl Future<Output = ResourceModelCollectionResult<BusinessUnitModel>>;

    fn delete(
        &self,
        actor: &Actor,
        path: &BusinessUnitPath,
    ) -> impl Future<Output = RestServiceResult<()>>;

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

impl<CB: CommandBus, QB: QueryBus> BusinessUnitRestService
    for RestService<CB, QB>
{
    async fn get(
        &self,
        actor: &Actor,
        path: &BusinessUnitPath,
    ) -> ResourceModelResult<BusinessUnitModel> {
        self.try_get(actor, path)
            .await?
            .ok_or_else(|| ApiError::NotFound)
    }

    async fn try_get(
        &self,
        actor: &Actor,
        path: &BusinessUnitPath,
    ) -> Result<Option<ResourceModel<BusinessUnitModel>>, ApiError> {
        // let query_result =
        //     query(self.query_bus(), &BusinessUnitQuery::ById(path.id()))
        //         .await?;
        // Ok(query_result.first().map(|bu| bu.clone().into()))
        todo!()
    }

    async fn query(
        &self,
        actor: &Actor,
        path: &BusinessUnitCollectionPath,
    ) -> ResourceModelCollectionResult<BusinessUnitModel> {
        // let bus = query(self.query_bus(), &BusinessUnitQuery::All).await?;
        // Ok(bus.into())
        todo!()
    }

    async fn delete(
        &self,
        actor: &Actor,
        path: &BusinessUnitPath,
    ) -> RestServiceResult<()> {
        Ok(self
            .command_bus()
            .execute::<_, DeleteBusinessUnitCommandHandler, _>(
                actor,
                &DeleteBusinessUnitCommand::builder()
                    .business_unit_id(path.id())
                    .build(),
            )
            .await
            .map(|_| ())?)
    }

    async fn update(
        &self,
        actor: &Actor,
        id: &BusinessUnitPath,
        payload: &UpdateBusinessUnitRequest,
    ) -> ResourceModelResult<BusinessUnitModel> {
        let cmd = UpdateBusinessUnitCommand::builder()
            .business_unit_id(id.id())
            .name(payload.name()?)
            .vars(payload.vars())
            .build();

        let bu = self
            .command_bus()
            .execute::<_, UpdateBusinessUnitCommandHandler, _>(actor, &cmd)
            .await?;

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
            .vars(payload.vars())
            .build();

        let bu = self
            .command_bus()
            .execute::<_, CreateBusinessUnitCommandHandler, _>(actor, &cmd)
            .await?;

        Ok(bu.into())
    }
}

async fn query<QB: QueryBus>(
    query_bus: &QB,
    query: &BusinessUnitQuery<'_>,
) -> Result<Vec<BusinessUnit>, QueryBusError> {
    // query_bus
    //     .execute::<_, QueryBusinessUnitsHandler, _>(&Actor::System, query)
    //     .await
    todo!()
}
