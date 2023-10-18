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
    find_business_unit::{FindBusinessUnitQueryBuilder, FindBusinessUnitQueryHandler},
    query_business_units::{QueryBusinessUnitsQueryBuilder, QueryBusinessUnitsQueryHandler},
};
use tap::TapFallible;

pub type SingleResult = ApiResult<SingleResourceModel<BusinessUnitResource>>;
pub type CollectionResult = ApiResult<CollectionResourceModel<BusinessUnitResource>>;

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
        let command = CreateBusinessUnitCommandBuilder::default()
            .id(Id::new())
            .code(body.code()?)
            .name(body.name()?)
            .vars(body.vars()?)
            .build()
            .tap_err(|e| tracing::error!("Failed to build CreateBusinessUnitCommand: {e}"))
            .map_err(anyhow::Error::new)?;

        Ok(state
            .command_bus()
            .execute::<CreateBusinessUnitCommand, _>(actor, command)
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
        let command = FindBusinessUnitQueryBuilder::default()
            .id(path.into_inner().try_into()?)
            .build()
            .tap_err(|e| tracing::error!("Failed to build FindBusinessUnitByCodeQuery: {e}"))
            .map_err(anyhow::Error::new)?;

        Ok(state
            .query_bus()
            .execute::<_, FindBusinessUnitQueryHandler, _>(&actor, &command)
            .await
            .tap_err(|e| tracing::error!("Failed to retrieve business unit: {e}"))
            .map(ApiResponse::ok)?)
    }

    #[tracing::instrument(skip(state))]
    pub async fn query(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
    ) -> CollectionResult {
        let command = QueryBusinessUnitsQueryBuilder::default()
            .build()
            .tap_err(|e| tracing::error!("Failed to build QueryBusinessUnitsQuery: {e}"))
            .map_err(anyhow::Error::new)?;
        state
            .query_bus()
            .execute::<_, QueryBusinessUnitsQueryHandler, _>(&actor, &command)
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
        let command = UpdateBusinessUnitCommandBuilder::default()
            .business_unit_id(path.into_inner().try_into()?)
            .name(body.name())
            .vars(body.vars().map(Into::into))
            .build()
            .tap_err(|e| tracing::error!("Failed to build UpdateBusinessUnitCommand: {e}"))
            .map_err(anyhow::Error::new)?;

        Ok(state
            .command_bus()
            .execute::<UpdateBusinessUnitCommand, _>(actor, command)
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
        let command = DeleteBusinessUnitCommandBuilder::default()
            .business_unit_id(path.into_inner().try_into()?)
            .build()
            .tap_err(|e| tracing::error!("Failed to build DeleteBusinessUnitCommand: {e}"))
            .map_err(anyhow::Error::new)?;

        Ok(state
            .command_bus()
            .execute::<DeleteBusinessUnitCommand, _>(actor, command)
            .await
            .tap_err(|e: &CommandBusError| {
                tracing::error!("Failed to delete Business unit: {e}");
            })
            .map(|_| ApiResponse::ok_empty())?)
    }
}
