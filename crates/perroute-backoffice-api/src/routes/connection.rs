use crate::{
    api::{
        models::connection::{
            ConnectionResource, CreateConnectionRequest, UpdateConnectionRequest,
        },
        response::{ApiResponse, ApiResult, CollectionResourceModel, SingleResourceModel},
    },
    app::AppState,
    extractors::actor::ActorExtractor,
};
use actix_web::{
    web::{Data, Json, Path},
    HttpResponse,
};
use anyhow::Context;
use perroute_commons::{new_id, types::id::Id};
use perroute_cqrs::command_bus::handlers::connection::{
    create_connection::{CreateConnectionCommandBuilder, CreateConnectionCommandHandler},
    update_connection::{UpdateConnectionCommandBuilder, UpdateConnectionCommandHandler},
};

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
        let connection = state
            .command_bus()
            .execute::<_, CreateConnectionCommandHandler, _>(
                &actor,
                &CreateConnectionCommandBuilder::default()
                    .id(new_id!())
                    .name(body.name)
                    .plugin_id(body.plugin_id.try_into().context("Invalid plugin id")?)
                    .properties(body.properties.into())
                    .build()
                    .context("Failed to build command")?,
            )
            .await
            .unwrap();

        Ok(ApiResponse::ok(connection))
    }

    pub async fn update(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        Json(body): Json<UpdateConnectionRequest>,
        path: Path<Id>,
    ) -> SingleResult {
        let conn = state
            .command_bus()
            .execute::<_, UpdateConnectionCommandHandler, _>(
                &actor,
                &UpdateConnectionCommandBuilder::default()
                    .id(path.into_inner())
                    .name(body.name)
                    .properties(body.properties.map(Into::into))
                    .enabled(body.enabled)
                    .build()
                    .context("Failed to build command")?,
            )
            .await?;

        Ok(ApiResponse::ok(conn))
    }

    pub async fn delete(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        path: Path<Id>,
    ) -> HttpResponse {
        HttpResponse::Ok().finish()
    }

    pub async fn get(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        path: Path<Id>,
    ) -> HttpResponse {
        HttpResponse::Ok().finish()
    }

    pub async fn query(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
    ) -> HttpResponse {
        HttpResponse::Ok().finish()
    }
}
