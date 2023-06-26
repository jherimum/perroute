use crate::rest::{
    api_models::schema::{CreateSchemaRequest, SchemaResource, UpdateSchemaRequest},
    extractors::{
        actor::ActorExtractor, message_type::MessageTypeExtractor, schema::SchemaExtractor,
    },
    Buses,
};
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::{delete, get, post, put},
    Json, Router,
};
use perroute_commons::{new_id, rest::RestError, types::id::Id};
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
    query_bus::{
        bus::QueryBus,
        handlers::schema::{
            find_schema::FindSchemaQueryHandler, query_schemas::QuerySchemasQueryHandler,
        },
        queries::{FindSchemaQueryBuilder, QuerySchemasQueryBuilder},
    },
};
use std::ops::Deref;

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
        ActorExtractor(actor): ActorExtractor,
        message_type: MessageTypeExtractor<Path<(Id, Id)>>,
        State(query_bus): State<QueryBus>,
    ) -> Result<Json<Vec<SchemaResource>>, RestError> {
        let query = QuerySchemasQueryBuilder::default()
            .message_type_id(*message_type.id())
            .build()
            .unwrap();

        Ok(Json(
            query_bus
                .execute::<_, QuerySchemasQueryHandler, _>(&actor, query)
                .await
                .unwrap()
                .into_iter()
                .map(SchemaResource::from)
                .collect::<Vec<_>>(),
        ))
    }

    async fn create_schema(
        ActorExtractor(actor): ActorExtractor,
        message_type: MessageTypeExtractor<Path<(Id, Id)>>,
        State(query_bus): State<QueryBus>,
        State(command_bus): State<CommandBus>,
        Json(body): Json<CreateSchemaRequest>,
    ) -> Result<Json<SchemaResource>, RestError> {
        let id = new_id!();
        let cmd = CreateSchemaCommandBuilder::default()
            .schema_id(id)
            .message_type_id(*message_type.id())
            .schema(body.schema)
            .build()
            .unwrap();

        command_bus
            .execute::<_, CreateSchemaCommandHandler>(&actor, cmd)
            .await
            .unwrap();

        let query = FindSchemaQueryBuilder::default()
            .message_type_id(id)
            .build()
            .unwrap();
        let schema = query_bus
            .execute::<_, FindSchemaQueryHandler, _>(&actor, query)
            .await
            .unwrap()
            .unwrap();

        Ok(Json(SchemaResource::from(schema)))
    }

    async fn update_schema(
        ActorExtractor(actor): ActorExtractor,
        schema: SchemaExtractor<Path<(Id, Id, i32)>>,
        State(query_bus): State<QueryBus>,
        State(command_bus): State<CommandBus>,
        Json(body): Json<UpdateSchemaRequest>,
    ) -> Result<Json<SchemaResource>, RestError> {
        let cmd = UpdateSchemaCommandBuilder::default()
            .schema_id(*schema.id())
            .schema(body.schema)
            .build()
            .unwrap();

        command_bus
            .execute::<_, UpdateSchemaCommandHandler>(&actor, cmd)
            .await
            .unwrap();

        let query = FindSchemaQueryBuilder::default()
            .message_type_id(*schema.message_type_id())
            .version(schema.deref().version().into())
            .build()
            .unwrap();
        let schema = query_bus
            .execute::<_, FindSchemaQueryHandler, _>(&actor, query)
            .await
            .unwrap()
            .unwrap();

        Ok(Json(SchemaResource::from(schema)))
    }

    async fn delete_schema(
        ActorExtractor(actor): ActorExtractor,
        schema: SchemaExtractor<Path<(Id, Id, i32)>>,
        State(command_bus): State<CommandBus>,
    ) -> Result<(), RestError> {
        let cmd = DeleteSchemaCommandBuilder::default()
            .schema_id(*schema.deref().id())
            .build()
            .unwrap();

        command_bus
            .execute::<_, DeleteSchemaCommandHandler>(&actor, cmd)
            .await
            .unwrap();

        Ok(())
    }

    async fn find_schema(
        schema: SchemaExtractor<Path<(Id, Id, i32)>>,
    ) -> Result<Json<SchemaResource>, RestError> {
        Ok(Json(SchemaResource::from(schema.deref())))
    }

    async fn publish_schema(
        Path((channel_id, message_type_id, shcema_id)): Path<(Id, Id, Id)>,
        State(query_bus): State<QueryBus>,
        State(command_bus): State<CommandBus>,
    ) -> impl IntoResponse {
        todo!()
    }
}
