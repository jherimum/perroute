use crate::rest::{
    api_models::schema::{CreateSchemaRequest, UpdateSchemaRequest},
    extractors::actor::ActorExtractor,
    Buses,
};
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::{delete, get, post, put},
    Json, Router,
};
use perroute_commons::{new_id, types::id::Id};
use perroute_cqrs::{
    command_bus::{
        bus::CommandBus,
        commands::{
            CreateSchemaCommandBuilder, DeleteSchemaCommandBuilder, UpdateSchemaCommandBuilder,
        },
        handlers::schema::{
            create_schema::CreateSchemaCommandHandler, delete_schema::DeleteSchemaCommandHandler,
            update_schema::UpdateSchemaCommandHandler,
        },
    },
    query_bus::bus::QueryBus,
};

pub struct SchemaRouter;

impl SchemaRouter {
    pub fn routes(buses: Buses) -> Router {
        Router::new()
            .nest(
                "/v1/channels/:channel_id/message_types/:message_type_id/schemas",
                Router::new()
                    .route("/", get(Self::query_schemas))
                    .route("/", post(Self::create_schema))
                    .nest(
                        "/:version",
                        Router::new()
                            .route("/", get(Self::find_schema))
                            .route("/", put(Self::update_schema))
                            .route("/", delete(Self::delete_schema))
                            .route("/publish", post(Self::publish_schema)),
                    ),
            )
            .with_state(buses)
    }

    async fn query_schemas(
        Path((channel_id, message_type_id)): Path<(Id, Id)>,
        State(query_bus): State<QueryBus>,
    ) -> impl IntoResponse {
        todo!()
    }

    async fn create_schema(
        ActorExtractor(actor): ActorExtractor,
        Path((channel_id, message_type_id)): Path<(Id, Id)>,
        State(query_bus): State<QueryBus>,
        State(command_bus): State<CommandBus>,
        Json(body): Json<CreateSchemaRequest>,
    ) -> impl IntoResponse {
        let cmd = CreateSchemaCommandBuilder::default()
            .schema_id(new_id!())
            .message_type_id(message_type_id)
            .schema(body.schema)
            .build()
            .unwrap();

        command_bus
            .execute::<_, CreateSchemaCommandHandler>(&actor, cmd)
            .await
            .unwrap();

        todo!()
    }

    async fn update_schema(
        ActorExtractor(actor): ActorExtractor,
        Path((channel_id, message_type_id, schema_id)): Path<(Id, Id, Id)>,
        State(query_bus): State<QueryBus>,
        State(command_bus): State<CommandBus>,
        Json(body): Json<UpdateSchemaRequest>,
    ) -> impl IntoResponse {
        let cmd = UpdateSchemaCommandBuilder::default()
            .schema_id(schema_id)
            .schema(body.schema)
            .build()
            .unwrap();

        command_bus
            .execute::<_, UpdateSchemaCommandHandler>(&actor, cmd)
            .await
            .unwrap();

        todo!()
    }

    async fn delete_schema(
        ActorExtractor(actor): ActorExtractor,
        Path((channel_id, message_type_id, schema_id)): Path<(Id, Id, Id)>,
        State(query_bus): State<QueryBus>,
        State(command_bus): State<CommandBus>,
    ) -> impl IntoResponse {
        let cmd = DeleteSchemaCommandBuilder::default()
            .schema_id(schema_id)
            .build()
            .unwrap();

        command_bus
            .execute::<_, DeleteSchemaCommandHandler>(&actor, cmd)
            .await
            .unwrap();

        todo!()
    }

    async fn find_schema(
        Path((channel_id, message_type_id, shcema_id)): Path<(Id, Id, Id)>,
        State(query_bus): State<QueryBus>,
    ) -> impl IntoResponse {
        todo!()
    }

    async fn publish_schema(
        Path((channel_id, message_type_id, shcema_id)): Path<(Id, Id, Id)>,
        State(query_bus): State<QueryBus>,
        State(command_bus): State<CommandBus>,
    ) -> impl IntoResponse {
        todo!()
    }
}
