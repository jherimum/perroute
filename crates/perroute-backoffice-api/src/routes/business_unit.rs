use crate::{
    api::{
        models::business_unit::{
            BusinessUnitResource, CreateBusinessUnitRequest, UpdateBusinessUnitRequest,
        },
        response::{
            ApiResponse, ApiResult, CollectionResourceModel, EmptyApiResult, SingleResourceModel,
        },
        types::SingleIdPath,
    },
    app::AppState,
    error::ApiError,
    extractors::actor::ActorExtractor,
    links::ResourceLink,
    W,
};
use actix_web::web::Data;
use actix_web_validator::{Json, Path};
use perroute_commandbus::{
    command::business_unit::{
        create_business_unit::{CreateBusinessUnitCommand, CreateBusinessUnitCommandBuilder},
        delete_business_unit::{DeleteBusinessUnitCommand, DeleteBusinessUnitCommandBuilder},
        update_business_unit::{UpdateBusinessUnitCommand, UpdateBusinessUnitCommandBuilder},
    },
    error::CommandBusError,
};
use perroute_commons::types::id::Id;

use perroute_cqrs::query_bus::handlers::business_unit::{
    find_business_unit::{
        FindBusinessUnitQuery, FindBusinessUnitQueryBuilder, FindBusinessUnitQueryHandler,
    },
    query_business_units::{
        QueryBusinessUnitsQuery, QueryBusinessUnitsQueryBuilder, QueryBusinessUnitsQueryHandler,
    },
};
use tap::TapFallible;

pub type SingleResult = ApiResult<SingleResourceModel<BusinessUnitResource>>;
pub type CollectionResult = ApiResult<CollectionResourceModel<BusinessUnitResource>>;

impl TryInto<CreateBusinessUnitCommand> for CreateBusinessUnitRequest {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<CreateBusinessUnitCommand, Self::Error> {
        Ok(CreateBusinessUnitCommandBuilder::default()
            .id(Id::new())
            .code(self.code()?)
            .name(self.name()?)
            .vars(self.vars()?)
            .build()
            .tap_err(|e| tracing::error!("Failed to build CreateBusinessUnitCommand: {e}"))?)
    }
}

impl TryInto<UpdateBusinessUnitCommand> for W<(Path<SingleIdPath>, UpdateBusinessUnitRequest)> {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<UpdateBusinessUnitCommand, Self::Error> {
        let w = self.into_inner();
        Ok(UpdateBusinessUnitCommandBuilder::default()
            .business_unit_id(w.0.into_inner().try_into()?)
            .name(w.1.name())
            .vars(w.1.vars().map(Into::into))
            .build()
            .tap_err(|e| tracing::error!("Failed to build UpdateBusinessUnitCommand: {e}"))?)
    }
}

impl TryInto<FindBusinessUnitQuery> for W<Path<SingleIdPath>> {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<FindBusinessUnitQuery, Self::Error> {
        Ok(FindBusinessUnitQueryBuilder::default()
            .id(self.into_inner().into_inner().try_into()?)
            .build()
            .tap_err(|e| tracing::error!("Failed to build FindBusinessUnitByCodeQuery: {e}"))?)
    }
}

impl TryInto<DeleteBusinessUnitCommand> for W<Path<SingleIdPath>> {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<DeleteBusinessUnitCommand, Self::Error> {
        Ok(DeleteBusinessUnitCommandBuilder::default()
            .business_unit_id(self.into_inner().into_inner().try_into()?)
            .build()
            .tap_err(|e| tracing::error!("Failed to build DeleteBusinessUnitCommand: {e}"))?)
    }
}

impl TryInto<QueryBusinessUnitsQuery> for W<()> {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<QueryBusinessUnitsQuery, Self::Error> {
        Ok(QueryBusinessUnitsQueryBuilder::default()
            .build()
            .tap_err(|e| tracing::error!("Failed to build QueryBusinessUnitsQuery: {e}"))?)
    }
}

pub struct BusinessUnitRouter;

impl BusinessUnitRouter {
    pub const BU_RESOURCE_NAME: &str = "business_unit";
    pub const BUS_RESOURCE_NAME: &str = "business_units";

    #[tracing::instrument(skip(state))]
    pub async fn create(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        Json(body): Json<CreateBusinessUnitRequest>,
    ) -> SingleResult {
        Ok(state
            .command_bus()
            .execute::<CreateBusinessUnitCommand, _>(actor, body.try_into()?)
            .await
            .tap_err(|e| tracing::error!("Failed to create bu: {e}"))
            .map(|bu| ApiResponse::created(ResourceLink::BusinessUnit(*bu.id()), bu))?)
    }

    #[tracing::instrument(skip(state))]
    pub async fn get(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        path: Path<SingleIdPath>,
    ) -> SingleResult {
        Ok(state
            .query_bus()
            .execute::<_, FindBusinessUnitQueryHandler, _>(&actor, &W(path).try_into()?)
            .await
            .tap_err(|e| tracing::error!("Failed to retrieve business unit: {e}"))
            .map(ApiResponse::ok)?)
    }

    #[tracing::instrument(skip(state))]
    pub async fn query(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
    ) -> CollectionResult {
        state
            .query_bus()
            .execute::<_, QueryBusinessUnitsQueryHandler, _>(&actor, &W(()).try_into()?)
            .await
            .tap_err(|e| tracing::error!("Failed to query business units: {e}"))
            .map(ApiResponse::ok)
            .map_err(ApiError::from)
    }

    #[tracing::instrument(skip(state))]
    pub async fn update(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        path: Path<SingleIdPath>,
        Json(body): Json<UpdateBusinessUnitRequest>,
    ) -> SingleResult {
        Ok(state
            .command_bus()
            .execute::<UpdateBusinessUnitCommand, _>(actor, W((path, body)).try_into()?)
            .await
            .tap_err(|e| tracing::error!("Failed to update BusinessUnit: {e}"))
            .map(ApiResponse::ok)?)
    }

    #[tracing::instrument(skip(state))]
    pub async fn delete(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        path: Path<SingleIdPath>,
    ) -> EmptyApiResult {
        Ok(state
            .command_bus()
            .execute::<DeleteBusinessUnitCommand, _>(actor, W(path).try_into()?)
            .await
            .tap_err(|e: &CommandBusError| {
                tracing::error!("Failed to delete Business unit: {e}");
            })
            .map(|_| ApiResponse::ok_empty())?)
    }
}
