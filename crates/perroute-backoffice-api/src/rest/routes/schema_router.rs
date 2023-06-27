use crate::rest::{
    api_models::schema::{CreateSchemaRequest, SchemaResource, UpdateSchemaRequest},
    extractors::{
        actor::ActorExtractor,
        resource_path::{MessageTypePath, ResourcePath, SchemaPath},
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
        bus::QueryBus, handlers::schema::query_schemas::QuerySchemasQueryHandler,
        queries::QuerySchemasQueryBuilder,
    },
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
        ActorExtractor(actor): ActorExtractor,
        message_type_path: MessageTypePath,
        State(query_bus): State<QueryBus>,
    ) -> Result<Json<Vec<SchemaResource>>, RestError> {
        let message_type = message_type_path
            .resource(&query_bus, &actor)
            .await?
            .ok_or(RestError::NotFound("".to_owned()))?;

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
        message_type_path: MessageTypePath,
        State(query_bus): State<QueryBus>,
        State(command_bus): State<CommandBus>,
        Json(body): Json<CreateSchemaRequest>,
    ) -> Result<Json<SchemaResource>, RestError> {
        let message_type = message_type_path
            .resource(&query_bus, &actor)
            .await?
            .ok_or(RestError::NotFound("".to_owned()))?;
        let cmd = CreateSchemaCommandBuilder::default()
            .schema_id(new_id!())
            .message_type_id(*message_type.id())
            .schema(body.schema)
            .build()
            .unwrap();

        command_bus
            .execute::<_, CreateSchemaCommandHandler>(&actor, cmd.clone())
            .await
            .unwrap();

        Ok(Json(SchemaResource::from(
            SchemaPath::from((
                *message_type.channel_id(),
                *message_type.id(),
                *cmd.schema_id(),
            ))
            .resource(&query_bus, &actor)
            .await?
            .ok_or(RestError::NotFound("".to_owned()))?,
        )))
    }

    async fn update_schema(
        ActorExtractor(actor): ActorExtractor,
        schema_path: SchemaPath,
        State(query_bus): State<QueryBus>,
        State(command_bus): State<CommandBus>,
        Json(body): Json<UpdateSchemaRequest>,
    ) -> Result<Json<SchemaResource>, RestError> {
        let schema = schema_path
            .resource(&query_bus, &actor)
            .await?
            .ok_or(RestError::NotFound("".to_owned()))?;
        let cmd = UpdateSchemaCommandBuilder::default()
            .schema_id(*schema.id())
            .schema(body.schema)
            .build()
            .unwrap();

        command_bus
            .execute::<_, UpdateSchemaCommandHandler>(&actor, cmd)
            .await
            .unwrap();

        Ok(Json(SchemaResource::from(
            SchemaPath::from((
                schema_path.channel_id,
                schema_path.message_type_id,
                schema_path.schema_id,
            ))
            .resource(&query_bus, &actor)
            .await?
            .ok_or(RestError::NotFound("".to_owned()))?,
        )))
    }

    async fn delete_schema(
        ActorExtractor(actor): ActorExtractor,
        schema_path: SchemaPath,
        State(query_bus): State<QueryBus>,
        State(command_bus): State<CommandBus>,
    ) -> Result<(), RestError> {
        let schema = schema_path
            .resource(&query_bus, &actor)
            .await?
            .ok_or(RestError::NotFound("".to_owned()))?;
        let cmd = DeleteSchemaCommandBuilder::default()
            .schema_id(*schema.id())
            .build()
            .unwrap();

        command_bus
            .execute::<_, DeleteSchemaCommandHandler>(&actor, cmd)
            .await
            .unwrap();

        Ok(())
    }

    async fn find_schema(
        ActorExtractor(actor): ActorExtractor,
        schema_path: SchemaPath,
        State(query_bus): State<QueryBus>,
    ) -> Result<Json<SchemaResource>, RestError> {
        let schema = schema_path
            .resource(&query_bus, &actor)
            .await?
            .ok_or(RestError::NotFound("".to_owned()))?;
        Ok(Json(SchemaResource::from(schema)))
    }

    async fn publish_schema(
        Path((channel_id, message_type_id, shcema_id)): Path<(Id, Id, Id)>,
        State(query_bus): State<QueryBus>,
        State(command_bus): State<CommandBus>,
    ) -> impl IntoResponse {
        todo!()
    }
}
