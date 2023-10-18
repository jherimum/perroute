use crate::{
    api::{
        models::connection::{
            ConnectionResource, CreateConnectionRequest, UpdateConnectionRequest,
        },
        response::{
            ApiResponse, ApiResult, CollectionResourceModel, EmptyApiResult, SingleResourceModel,
        },
        types::SingleIdPath,
    },
    app::AppState,
    extractors::actor::ActorExtractor,
    links::ResourceLink,
};
use actix_web::web::Data;
use actix_web_validator::Json;
use actix_web_validator::Path;
use anyhow::Context;
use perroute_commandbus::command::connection::{
    create_connection::{CreateConnectionCommand, CreateConnectionCommandBuilder},
    delete_connection::{DeleteConnectionCommand, DeleteConnectionCommandBuilder},
    update_connection::{UpdateConnectionCommand, UpdateConnectionCommandBuilder},
};
use perroute_commons::types::id::Id;
use perroute_cqrs::query_bus::handlers::connection::{
    find_connection::{FindConnectionQueryBuilder, FindConnectionQueryHandler},
    query_connections::{QueryConnectionsQueryBuilder, QueryConnectionsQueryHandler},
};
use tap::TapFallible;

pub type SingleResult = ApiResult<SingleResourceModel<ConnectionResource>>;
pub type CollectionResult = ApiResult<CollectionResourceModel<ConnectionResource>>;

pub struct ConnectionsRouter;

impl ConnectionsRouter {
    pub const CONN_RESOURCE_NAME: &str = "connection";
    pub const CONNS_RESOURCE_NAME: &str = "connections";

    pub async fn create(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        Json(body): Json<CreateConnectionRequest>,
    ) -> SingleResult {
        let command = CreateConnectionCommandBuilder::default()
            .id(Id::new())
            .name(body.name()?)
            .plugin_id(body.plugin_id()?)
            .properties(body.properties()?)
            .build()
            .map_err(anyhow::Error::new)?;

        Ok(state
            .command_bus()
            .execute::<CreateConnectionCommand, _>(actor, command)
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
        let command = UpdateConnectionCommandBuilder::default()
            .id(path.into_inner().try_into()?)
            .name(body.name())
            .properties(body.properties()?)
            .enabled(body.enabled())
            .build()
            .context("Failed to build command")?;

        Ok(state
            .command_bus()
            .execute::<UpdateConnectionCommand, _>(actor, command)
            .await
            .tap_err(|e| tracing::error!("Failed to update connection: {e}"))
            .map(ApiResponse::ok)?)
    }

    pub async fn delete(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        path: Path<SingleIdPath>,
    ) -> EmptyApiResult {
        let command = DeleteConnectionCommandBuilder::default()
            .id(path.into_inner().try_into()?)
            .build()
            .map_err(anyhow::Error::new)?;
        Ok(state
            .command_bus()
            .execute::<DeleteConnectionCommand, _>(actor, command)
            .await
            .tap_err(|e| tracing::error!("Failed to delete connection: {e}"))
            .map(|_| ApiResponse::ok_empty())?)
    }

    pub async fn get(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        path: Path<SingleIdPath>,
    ) -> SingleResult {
        let query = FindConnectionQueryBuilder::default()
            .id(path.into_inner().try_into()?)
            .build()
            .map_err(anyhow::Error::new)?;

        Ok(state
            .query_bus()
            .execute::<_, FindConnectionQueryHandler, _>(&actor, &query)
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
