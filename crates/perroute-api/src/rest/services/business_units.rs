use super::DefaultRestService;
use crate::rest::{
    error::ApiError,
    models::{ResourceModel, ResourceModelCollection},
    routes::business_units::models::{
        BusinessUnitModel, BusinessUnitPath, CreateBusinessUnitRequest, UpdateBusinessUnitRequest,
    },
    EmptyResourceModelResult, ResourceModelCollectionResult, ResourceModelResult,
    RestServiceResult,
};
use perroute_command_bus::CommandBus;
use perroute_commons::types::actor::Actor;
use perroute_query_bus::{queries::business_unit::QueryBusinessUnitsHandler, QueryBus};
use perroute_storage::{
    models::business_unit::BusinessUnit, repository::business_units::BusinessUnitQuery,
};
use std::future::Future;

pub trait BusinessUnitRestService {
    fn get(
        &self,
        id: &BusinessUnitPath,
    ) -> impl Future<Output = ResourceModelResult<BusinessUnitModel>>;

    fn query(&self) -> impl Future<Output = ResourceModelCollectionResult<BusinessUnitModel>>;

    fn delete(&self, id: &BusinessUnitPath) -> impl Future<Output = EmptyResourceModelResult>;

    fn update(
        &self,
        id: &BusinessUnitPath,
        payload: &UpdateBusinessUnitRequest,
    ) -> impl Future<Output = ResourceModelResult<BusinessUnitModel>>;

    fn create(
        &self,
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

impl<CB: CommandBus, QB: QueryBus> BusinessUnitRestService for DefaultRestService<CB, QB> {
    async fn get(&self, id: &BusinessUnitPath) -> ResourceModelResult<BusinessUnitModel> {
        let query_result = query(&self.query_bus, &BusinessUnitQuery::ById(id.into())).await?;
        let bu = query_result.first().ok_or_else(|| ApiError::NotFound)?;
        Ok(ResourceModel::new(BusinessUnitModel::from(bu)))
    }

    async fn query(&self) -> ResourceModelCollectionResult<BusinessUnitModel> {
        let query_result = query(&self.query_bus, &BusinessUnitQuery::All).await?;

        Ok(ResourceModelCollection {
            data: query_result
                .iter()
                .map(BusinessUnitModel::from)
                .map(ResourceModel::new)
                .collect::<Vec<_>>(),
        })
    }

    async fn delete(&self, id: &BusinessUnitPath) -> EmptyResourceModelResult {
        todo!()
    }

    async fn update(
        &self,
        id: &BusinessUnitPath,
        payload: &UpdateBusinessUnitRequest,
    ) -> ResourceModelResult<BusinessUnitModel> {
        todo!()
    }

    async fn create(
        &self,
        payload: &CreateBusinessUnitRequest,
    ) -> ResourceModelResult<BusinessUnitModel> {
        todo!()
    }
}
