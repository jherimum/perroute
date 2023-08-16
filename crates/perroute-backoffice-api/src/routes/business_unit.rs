use crate::{
    api::{
        models::business_unit::{
            BusinessUnitResource, CreateBusinessUnitRequest, UpdateBusinessUnitRequest,
        },
        response::{
            ApiResponse, ApiResult, CollectionResourceModel, EmptyApiResult, SingleResourceModel,
        },
    },
    app::AppState,
    error::ApiError,
    extractors::actor::ActorExtractor,
    links::ResourceLink,
};
use actix_web::web::{Data, Json, Path};
use perroute_commons::types::{actor::Actor, id::Id};
use perroute_cqrs::{
    command_bus::handlers::business_unit::{
        create_business_unit::{
            CreateBusinessUnitCommandBuilder, CreateBusinessUnitCommandHandler,
        },
        delete_business_unit::{
            DeleteBusinessUnitCommandBuilder, DeleteBusinessUnitCommandHandler,
        },
        update_business_unit::{
            UpdateBusinessUnitCommandBuilder, UpdateBusinessUnitCommandHandler,
        },
    },
    query_bus::{
        bus::QueryBus,
        handlers::business_unit::{
            find_business_unit::{FindBusinessUnitQueryBuilder, FindBusinessUnitQueryHandler},
            query_business_units::{
                QueryBusinessUnitsQueryBuilder, QueryBusinessUnitsQueryHandler,
            },
        },
    },
};
use perroute_storage::models::business_unit::BusinessUnit;
use std::convert::identity;
use tap::TapFallible;

pub type SingleResult = ApiResult<SingleResourceModel<BusinessUnitResource>>;
pub type CollectionResult = ApiResult<CollectionResourceModel<BusinessUnitResource>>;

pub struct BusinessUnitRouter;

impl BusinessUnitRouter {
    pub const BU_RESOURCE_NAME: &str = "business_unit";
    pub const BUS_RESOURCE_NAME: &str = "business_units";

    #[tracing::instrument(skip(state))]
    pub async fn create_bu(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        Json(body): Json<CreateBusinessUnitRequest>,
    ) -> SingleResult {
        let cmd = CreateBusinessUnitCommandBuilder::default()
            .code(body.code().clone())
            .name(body.name().clone())
            .vars(body.vars().clone())
            .build()
            .tap_err(|e| tracing::error!("Failed to build CreateBusinessUnitCommand: {e}"))
            .map_err(anyhow::Error::from)?;

        Ok(state
            .command_bus()
            .execute::<_, CreateBusinessUnitCommandHandler, _>(&actor, &cmd)
            .await
            .tap_err(|e| tracing::error!("Failed to create bu: {e}"))
            .map(|bu| ApiResponse::created(ResourceLink::BusinessUnit(*bu.id()), bu))?)
    }

    #[tracing::instrument(skip(state))]
    pub async fn find_bu(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        path: Path<Id>,
    ) -> SingleResult {
        Self::retrieve_bu(state.query_bus(), &actor, path.into_inner(), {
            ApiResponse::ok
        })
        .await
    }

    #[tracing::instrument(skip(state))]
    pub async fn query_bus(
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
    pub async fn update_bu(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        path: Path<Id>,
        Json(body): Json<UpdateBusinessUnitRequest>,
    ) -> SingleResult {
        let business_unit =
            Self::retrieve_bu(state.query_bus(), &actor, path.into_inner(), identity).await?;

        let cmd = UpdateBusinessUnitCommandBuilder::default()
            .business_unit_id(*business_unit.id())
            .name(body.name)
            .vars(body.vars)
            .build()
            .tap_err(|e| tracing::error!("Failed to build UpdateBusinessUnitCommand: {e}"))
            .map_err(anyhow::Error::from)?;

        Ok(state
            .command_bus()
            .execute::<_, UpdateBusinessUnitCommandHandler, _>(&actor, &cmd)
            .await
            .tap_err(|e| tracing::error!("Failed to update BusinessUnit: {e}"))
            .map(ApiResponse::ok)?)
    }

    #[tracing::instrument(skip(state))]
    pub async fn delete_bu(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        path: Path<Id>,
    ) -> EmptyApiResult {
        let business_unit =
            Self::retrieve_bu(state.query_bus(), &actor, path.into_inner(), identity).await?;

        let cmd = DeleteBusinessUnitCommandBuilder::default()
            .business_unit_id(*business_unit.id())
            .build()
            .tap_err(|e| tracing::error!("Failed to build DeleteBusinessUnitCommand: {e}"))
            .map_err(anyhow::Error::from)?;

        Ok(state
            .command_bus()
            .execute::<_, DeleteBusinessUnitCommandHandler, _>(&actor, &cmd)
            .await
            .tap_err(|e| tracing::error!("Failed to delete Business unit: {e}"))
            .map(|_| ApiResponse::ok_empty())?)
    }

    pub async fn retrieve_bu<R>(
        query_bus: &QueryBus,
        actor: &Actor,
        id: Id,
        map: impl FnOnce(BusinessUnit) -> R + Send + Sync,
    ) -> Result<R, ApiError> {
        let query = FindBusinessUnitQueryBuilder::default()
            .business_unit_id(Some(id))
            .build()
            .tap_err(|e| tracing::error!("Failed to build FindBusinessUnitByCodeQuery: {e}"))
            .map_err(anyhow::Error::from)?;

        query_bus
            .execute::<_, FindBusinessUnitQueryHandler, _>(actor, &query)
            .await
            .tap_err(|e| tracing::error!("Failed to retrieve business unit: {e}"))?
            .ok_or_else(|| ApiError::BusinessUnitNotFound(id))
            .map(map)
    }
}
