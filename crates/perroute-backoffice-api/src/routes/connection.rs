use crate::{
    api::models::connection::{CreateConnectionRequest, UpdateConnectionRequest},
    app::AppState,
    extractors::actor::ActorExtractor,
};
use actix_web::{
    web::{Data, Json, Path},
    HttpResponse,
};
use anyhow::Context;
use perroute_commons::{new_id, types::id::Id};
use perroute_cqrs::command_bus::handlers::connection::create_connection::{
    CreateConnectionCommandBuilder, CreateConnectionCommandHandler,
};

pub struct ConnectionsRouter;

impl ConnectionsRouter {
    pub const CONN_RESOURCE_NAME: &str = "connection";
    pub const CONNS_RESOURCE_NAME: &str = "connections";

    pub async fn create(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        Json(body): Json<CreateConnectionRequest>,
    ) -> HttpResponse {
        let connection = state
            .command_bus()
            .execute::<_, CreateConnectionCommandHandler, _>(
                &actor,
                &CreateConnectionCommandBuilder::default()
                    .id(new_id!())
                    .name(body.name().to_owned())
                    .plugin_id(
                        body.plugin_id()
                            .try_into()
                            .context("Invalid plugin id")
                            .unwrap(),
                    )
                    .properties(body.properties().into())
                    .build()
                    .unwrap(),
            )
            .await
            .unwrap();

        HttpResponse::Ok().finish()
    }

    pub async fn update(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        Json(body): Json<UpdateConnectionRequest>,
        path: Path<Id>,
    ) -> HttpResponse {
        HttpResponse::Ok().finish()
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
