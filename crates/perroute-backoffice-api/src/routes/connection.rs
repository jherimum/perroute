use crate::{
    api::{
        models::{
            connection::{ConnectionResource, CreateConnectionRequest, UpdateConnectionRequest},
            SingleIdPath,
        },
        response::{
            ApiResponse, ApiResult, CollectionResourceModel, EmptyApiResult, SingleResourceModel,
        },
    },
    app::AppState,
    extractors::actor::ActorExtractor,
    links::ResourceLink,
    W,
};
use actix_web::web::Data;
use actix_web_validator::Json;
use actix_web_validator::Path;
use anyhow::Context;
use perroute_commons::types::id::Id;
use perroute_cqrs::{
    command_bus::handlers::connection::{
        create_connection::{
            CreateConnectionCommand, CreateConnectionCommandBuilder, CreateConnectionCommandHandler,
        },
        delete_connection::{
            DeleteConnectionCommand, DeleteConnectionCommandBuilder, DeleteConnectionCommandHandler,
        },
        update_connection::{
            UpdateConnectionCommand, UpdateConnectionCommandBuilder, UpdateConnectionCommandHandler,
        },
    },
    query_bus::handlers::connection::{
        find_connection::{
            FindConnectionQuery, FindConnectionQueryBuilder, FindConnectionQueryHandler,
        },
        query_connections::{QueryConnectionsQueryBuilder, QueryConnectionsQueryHandler},
    },
};
use tap::TapFallible;

pub type SingleResult = ApiResult<SingleResourceModel<ConnectionResource>>;
pub type CollectionResult = ApiResult<CollectionResourceModel<ConnectionResource>>;

impl TryInto<CreateConnectionCommand> for CreateConnectionRequest {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<CreateConnectionCommand, Self::Error> {
        Ok(CreateConnectionCommandBuilder::default()
            .id(Id::new())
            .name(self.name.context("Missing name")?)
            .plugin_id(
                self.plugin_id
                    .context("Missing plugin id")?
                    .try_into()
                    .context("Invalid plugin id")?,
            )
            .properties(self.properties.context("Missing properties")?.into())
            .build()?)
    }
}

impl TryInto<UpdateConnectionCommand> for W<(SingleIdPath, UpdateConnectionRequest)> {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<UpdateConnectionCommand, Self::Error> {
        UpdateConnectionCommandBuilder::default()
            .id(self.0 .0.try_into()?)
            .name(self.0 .1.name)
            .properties(self.0 .1.properties.map(Into::into))
            .enabled(self.0 .1.enabled)
            .build()
            .context("Failed to build command")
    }
}

impl TryInto<DeleteConnectionCommand> for SingleIdPath {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<DeleteConnectionCommand, Self::Error> {
        Ok(DeleteConnectionCommandBuilder::default()
            .id(self.try_into()?)
            .build()?)
    }
}

impl TryInto<FindConnectionQuery> for SingleIdPath {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<FindConnectionQuery, Self::Error> {
        Ok(FindConnectionQueryBuilder::default()
            .id(self.try_into()?)
            .build()?)
    }
}

pub struct ConnectionsRouter;

impl ConnectionsRouter {
    pub const CONN_RESOURCE_NAME: &str = "connection";
    pub const CONNS_RESOURCE_NAME: &str = "connections";

    pub async fn create(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        Json(body): Json<CreateConnectionRequest>,
    ) -> SingleResult {
        Ok(state
            .command_bus()
            .execute::<_, CreateConnectionCommandHandler, _>(&actor, &body.try_into()?)
            .await
            .tap_err(|e| tracing::error!("Failed to create connection: {e}"))
            .map(|c| ApiResponse::created(ResourceLink::Connection(*c.id()), c))?)
    }

    pub async fn update(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        Json(body): Json<UpdateConnectionRequest>,
        path: Path<SingleIdPath>,
    ) -> SingleResult {
        Ok(state
            .command_bus()
            .execute::<_, UpdateConnectionCommandHandler, _>(
                &actor,
                &W((path.into_inner(), body)).try_into()?,
            )
            .await
            .tap_err(|e| tracing::error!("Failed to update connection: {e}"))
            .map(ApiResponse::ok)?)
    }

    pub async fn delete(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        path: Path<SingleIdPath>,
    ) -> EmptyApiResult {
        Ok(state
            .command_bus()
            .execute::<_, DeleteConnectionCommandHandler, _>(&actor, &path.into_inner().try_into()?)
            .await
            .tap_err(|e| tracing::error!("Failed to delete connection: {e}"))
            .map(|_| ApiResponse::ok_empty())?)
    }

    pub async fn get(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        path: Path<SingleIdPath>,
    ) -> SingleResult {
        Ok(state
            .query_bus()
            .execute::<_, FindConnectionQueryHandler, _>(&actor, &path.into_inner().try_into()?)
            .await
            .map(ApiResponse::ok)?)
    }

    pub async fn query(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
    ) -> CollectionResult {
        Ok(state
            .query_bus()
            .execute::<_, QueryConnectionsQueryHandler, _>(
                &actor,
                &QueryConnectionsQueryBuilder::default()
                    .build()
                    .context("Failed to build query")?,
            )
            .await
            .map(ApiResponse::ok)?)
    }
}
