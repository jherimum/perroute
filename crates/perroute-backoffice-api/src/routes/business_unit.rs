use crate::{
    api::{
        models::{
            business_unit::{
                BusinessUnitResource, CreateBusinessUnitRequest, UpdateBusinessUnitRequest,
            },
            SingleIdPath,
        },
        response::{
            ApiResponse, ApiResult, CollectionResourceModel, EmptyApiResult, SingleResourceModel,
        },
    },
    app::AppState,
    error::ApiError,
    extractors::actor::ActorExtractor,
    links::ResourceLink,
    W,
};
use actix_web::web::Data;
use actix_web_validator::{Json, Path};
use anyhow::Context;
use perroute_commons::types::{code::Code, id::Id};
use perroute_cqrs::{
    command_bus::handlers::business_unit::{
        create_business_unit::{
            CreateBusinessUnitCommand, CreateBusinessUnitCommandBuilder,
            CreateBusinessUnitCommandHandler,
        },
        delete_business_unit::{
            DeleteBusinessUnitCommand, DeleteBusinessUnitCommandBuilder,
            DeleteBusinessUnitCommandHandler,
        },
        update_business_unit::{
            UpdateBusinessUnitCommand, UpdateBusinessUnitCommandBuilder,
            UpdateBusinessUnitCommandHandler,
        },
    },
    query_bus::handlers::business_unit::{
        find_business_unit::{
            FindBusinessUnitQuery, FindBusinessUnitQueryBuilder, FindBusinessUnitQueryHandler,
        },
        query_business_units::{QueryBusinessUnitsQueryBuilder, QueryBusinessUnitsQueryHandler},
    },
};
use std::str::FromStr;
use tap::TapFallible;

pub type SingleResult = ApiResult<SingleResourceModel<BusinessUnitResource>>;
pub type CollectionResult = ApiResult<CollectionResourceModel<BusinessUnitResource>>;

impl TryInto<CreateBusinessUnitCommand> for CreateBusinessUnitRequest {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<CreateBusinessUnitCommand, Self::Error> {
        Ok(CreateBusinessUnitCommandBuilder::default()
            .id(Id::new())
            .code(Code::from_str(&self.code.context("Missing code")?).context("Invalid code")?)
            .name(self.name.context("Misssing name")?)
            .vars(self.vars.unwrap_or_default().into())
            .build()
            .tap_err(|e| tracing::error!("Failed to build CreateBusinessUnitCommand: {e}"))?)
    }
}

impl TryInto<UpdateBusinessUnitCommand> for W<(SingleIdPath, UpdateBusinessUnitRequest)> {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<UpdateBusinessUnitCommand, Self::Error> {
        let w = self.into_inner();
        Ok(UpdateBusinessUnitCommandBuilder::default()
            .business_unit_id(w.0.try_into().context("Invalid id")?)
            .name(w.1.name)
            .vars(w.1.vars.map(Into::into))
            .build()
            .tap_err(|e| tracing::error!("Failed to build UpdateBusinessUnitCommand: {e}"))?)
    }
}

impl TryInto<FindBusinessUnitQuery> for SingleIdPath {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<FindBusinessUnitQuery, Self::Error> {
        Ok(FindBusinessUnitQueryBuilder::default()
            .id(self.try_into().context("Invalid id")?)
            .build()
            .tap_err(|e| tracing::error!("Failed to build FindBusinessUnitByCodeQuery: {e}"))?)
    }
}

impl TryInto<DeleteBusinessUnitCommand> for SingleIdPath {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<DeleteBusinessUnitCommand, Self::Error> {
        Ok(DeleteBusinessUnitCommandBuilder::default()
            .business_unit_id(self.try_into().context("Invalid id")?)
            .build()
            .tap_err(|e| tracing::error!("Failed to build DeleteBusinessUnitCommand: {e}"))?)
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
            .execute::<_, CreateBusinessUnitCommandHandler, _>(&actor, &body.try_into()?)
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
            .execute::<_, FindBusinessUnitQueryHandler, _>(&actor, &path.into_inner().try_into()?)
            .await
            .tap_err(|e| tracing::error!("Failed to retrieve business unit: {e}"))
            .map(ApiResponse::ok)?)
    }

    #[tracing::instrument(skip(state))]
    pub async fn query(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
    ) -> CollectionResult {
        let query = QueryBusinessUnitsQueryBuilder::default()
            .build()
            .tap_err(|e| tracing::error!("Failed to build QueryBusinessUnitsQuery: {e}"))
            .map_err(anyhow::Error::from)?;
        state
            .query_bus()
            .execute::<_, QueryBusinessUnitsQueryHandler, _>(&actor, &query)
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
            .execute::<_, UpdateBusinessUnitCommandHandler, _>(
                &actor,
                &W((path.into_inner(), body)).try_into()?,
            )
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
            .execute::<_, DeleteBusinessUnitCommandHandler, _>(
                &actor,
                &path.into_inner().try_into()?,
            )
            .await
            .tap_err(|e: &perroute_cqrs::command_bus::error::CommandBusError| {
                tracing::error!("Failed to delete Business unit: {e}");
            })
            .map(|_| ApiResponse::ok_empty())?)
    }
}
