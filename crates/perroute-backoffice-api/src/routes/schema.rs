use super::message_type::MessageTypeRouter;
use crate::api::models::schema::UpdateSchemaRequest;
use crate::{
    api::{
        models::schema::{CreateSchemaRequest, SchemaResource},
        response::{ApiResponse, ApiResult, EmptyResource},
        ResourceLink,
    },
    app::AppState,
    error::ApiError,
    extractors::actor::ActorExtractor,
};
use actix_web::web::{Data, Json, Path};
use perroute_commons::{
    new_id,
    types::{actor::Actor, id::Id, json_schema::JsonSchema},
};
use perroute_cqrs::query_bus::queries::FindChannelMessageTypeSchemaQueryBuilder;
use perroute_cqrs::{
    command_bus::{
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
        handlers::schema::find_channel__message_type_schema::FindChannelMessageTypeSchemaQueryHandler,
    },
};
use perroute_storage::models::message_type::MessageType;
use perroute_storage::models::schema::Schema;
use std::convert::identity;

pub const SCHEMAS_RESOURCE_NAME: &str = "schemas";
pub const SCHEMA_RESOURCE_NAME: &str = "schema";

pub struct SchemaRouter;

impl SchemaRouter {
    pub async fn retrieve_schema<R>(
        query_bus: &QueryBus,
        actor: &Actor,
        channel_id: Id,
        message_type_id: Id,
        schema_id: Id,
        map: impl FnOnce(Schema) -> R,
    ) -> Result<R, ApiError> {
        let query = FindChannelMessageTypeSchemaQueryBuilder::default()
            .channel_id(channel_id)
            .message_type_id(message_type_id)
            .schema_id(schema_id)
            .build()
            .unwrap();

        query_bus
            .execute::<_, FindChannelMessageTypeSchemaQueryHandler, _>(actor, &query)
            .await
            .unwrap()
            .ok_or_else(|| ApiError::SchemaNotFound(schema_id))
            .map(map)
    }

    async fn retrieve_message_type(
        query_bus: &QueryBus,
        actor: &Actor,
        channel_id: Id,
        message_type_id: Id,
    ) -> Result<MessageType, ApiError> {
        MessageTypeRouter::retrieve_message_type(
            query_bus,
            actor,
            channel_id,
            message_type_id,
            identity,
        )
        .await
    }

    #[tracing::instrument]
    pub async fn query_schemas(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        schemas_path: Path<(Id, Id)>,
    ) -> ApiResult<EmptyResource> {
        Self::retrieve_message_type(state.query_bus(), &actor, schemas_path.0, schemas_path.1)
            .await?;

        //HttpResponse::Ok().finish()
        todo!()
    }

    #[tracing::instrument]
    pub async fn create_schema(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        schemas_path: Path<(Id, Id)>,
        Json(body): Json<CreateSchemaRequest>,
    ) -> ApiResult<SchemaResource> {
        let message_type =
            Self::retrieve_message_type(state.query_bus(), &actor, schemas_path.0, schemas_path.1)
                .await?;

        let cmd = CreateSchemaCommandBuilder::default()
            .schema_id(new_id!())
            .message_type_id(body.message_type_id)
            .schema(JsonSchema::try_from(body.schema).map_err(ApiError::from)?)
            .build()
            .unwrap();

        Ok(state
            .command_bus()
            .execute::<_, CreateSchemaCommandHandler, _>(&actor, &cmd)
            .await
            .map(|schema| {
                ApiResponse::Created(
                    ResourceLink::Schema(
                        *message_type.channel_id(),
                        *message_type.id(),
                        *schema.id(),
                    ),
                    schema.into(),
                )
            })?)
    }

    #[tracing::instrument]
    pub async fn update_schema(
        schema_path: Path<(Id, Id, Id)>,
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        Json(body): Json<UpdateSchemaRequest>,
    ) -> ApiResult<SchemaResource> {
        let schema = Self::retrieve_schema(
            state.query_bus(),
            &actor,
            schema_path.0,
            schema_path.1,
            schema_path.2,
            identity,
        )
        .await?;

        let cmd = UpdateSchemaCommandBuilder::default()
            .schema_id(*schema.id())
            .schema(JsonSchema::try_from(body.schema).map_err(ApiError::from)?)
            .build()
            .unwrap();

        Ok(state
            .command_bus()
            .execute::<_, UpdateSchemaCommandHandler, _>(&actor, &cmd)
            .await
            .map(|schema| {
                ApiResponse::Created(
                    ResourceLink::Schema(schema_path.0, schema_path.1, *schema.id()),
                    schema.into(),
                )
            })?)
    }

    #[tracing::instrument]
    pub async fn delete_schema(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        schema_path: Path<(Id, Id, Id)>,
    ) -> ApiResult<EmptyResource> {
        let schema = Self::retrieve_schema(
            state.query_bus(),
            &actor,
            schema_path.0,
            schema_path.1,
            schema_path.2,
            identity,
        )
        .await?;

        let cmd = DeleteSchemaCommandBuilder::default()
            .schema_id(*schema.id())
            .build()
            .unwrap();

        Ok(state
            .command_bus()
            .execute::<_, DeleteSchemaCommandHandler, _>(&actor, &cmd)
            .await
            .map(|_| ApiResponse::OkEmpty(EmptyResource))?)
    }

    #[tracing::instrument]
    pub async fn find_schema(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        schema_path: Path<(Id, Id, Id)>,
    ) -> ApiResult<SchemaResource> {
        Self::retrieve_schema(
            state.query_bus(),
            &actor,
            schema_path.0,
            schema_path.1,
            schema_path.2,
            |schema| {
                ApiResponse::Created(
                    ResourceLink::Schema(schema_path.0, schema_path.1, *schema.id()),
                    schema.into(),
                )
            },
        )
        .await
    }
}
