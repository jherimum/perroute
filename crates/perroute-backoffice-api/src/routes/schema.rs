use crate::api::models::schema::UpdateSchemaRequest;
use crate::routes::channel::ChannelRouter;
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
        bus::QueryBus, handlers::schema::find_channel_schema::FindChannelSchemaQueryHandler,
        queries::FindChannelSchemaQueryBuilder,
    },
};
use perroute_storage::models::channel::Channel;
use perroute_storage::models::schema::Schema;

pub const SCHEMAS_RESOURCE_NAME: &str = "schemas";
pub const SCHEMA_RESOURCE_NAME: &str = "schema";

pub struct SchemaRouter;

impl SchemaRouter {
    pub async fn retrieve_schema<R>(
        query_bus: &QueryBus,
        actor: &Actor,
        channel_id: Id,
        schema_id: Id,
        not_found: impl FnOnce() -> ApiError,
        map: impl FnOnce(Schema) -> R,
    ) -> Result<R, ApiError> {
        let query = FindChannelSchemaQueryBuilder::default()
            .channel_id(channel_id)
            .schema_id(schema_id)
            .build()
            .unwrap();

        query_bus
            .execute::<_, FindChannelSchemaQueryHandler, _>(actor, &query)
            .await
            .unwrap()
            .ok_or_else(not_found)
            .map(map)
    }

    async fn retrieve_channel(
        query_bus: &QueryBus,
        actor: &Actor,
        channel_id: Id,
    ) -> Result<Channel, ApiError> {
        ChannelRouter::retrieve_channel(query_bus, actor, &channel_id, || {
            ApiError::ChannelNotFound(channel_id)
        })
        .await
    }

    #[tracing::instrument]
    pub async fn query_schemas(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        schemas_path: Path<Id>,
    ) -> ApiResult<EmptyResource> {
        let channel =
            Self::retrieve_channel(state.query_bus(), &actor, schemas_path.into_inner()).await?;

        //HttpResponse::Ok().finish()
        todo!()
    }

    #[tracing::instrument]
    pub async fn create_schema(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        schemas_path: Path<Id>,
        Json(body): Json<CreateSchemaRequest>,
    ) -> ApiResult<SchemaResource> {
        let channel_id = schemas_path.into_inner();

        let channel = Self::retrieve_channel(state.query_bus(), &actor, channel_id).await?;

        let cmd = CreateSchemaCommandBuilder::default()
            .schema_id(new_id!())
            .message_type_id(body.message_type_id)
            .schema(JsonSchema::try_from(body.schema).map_err(ApiError::from)?)
            .build()
            .unwrap();

        state
            .command_bus()
            .execute::<_, CreateSchemaCommandHandler>(&actor, &cmd)
            .await?;

        Self::retrieve_schema(
            state.query_bus(),
            &actor,
            channel_id,
            *cmd.schema_id(),
            || ApiError::Const("()"),
            |schema| {
                ApiResponse::Created(
                    ResourceLink::Schema(*channel.id(), *schema.id()),
                    schema.into(),
                )
            },
        )
        .await
    }

    #[tracing::instrument]
    pub async fn update_schema(
        schema_path: Path<(Id, Id)>,
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        Json(body): Json<UpdateSchemaRequest>,
    ) -> ApiResult<SchemaResource> {
        let schema = Self::retrieve_schema(
            state.query_bus(),
            &actor,
            schema_path.0,
            schema_path.1,
            || ApiError::SchemaNotFound(schema_path.1),
            |s| s,
        )
        .await?;

        let cmd = UpdateSchemaCommandBuilder::default()
            .schema_id(*schema.id())
            .schema(JsonSchema::try_from(body.schema).map_err(ApiError::from)?)
            .build()
            .unwrap();

        state
            .command_bus()
            .execute::<_, UpdateSchemaCommandHandler>(&actor, &cmd)
            .await
            .unwrap();

        Self::retrieve_schema(
            state.query_bus(),
            &actor,
            schema_path.0,
            *cmd.schema_id(),
            || ApiError::Const("()"),
            |schema| {
                ApiResponse::Created(
                    ResourceLink::Schema(schema_path.0, *schema.id()),
                    schema.into(),
                )
            },
        )
        .await
    }

    #[tracing::instrument]
    pub async fn delete_schema(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        schema_path: Path<(Id, Id)>,
    ) -> ApiResult<EmptyResource> {
        let schema = Self::retrieve_schema(
            state.query_bus(),
            &actor,
            schema_path.0,
            schema_path.1,
            || ApiError::SchemaNotFound(schema_path.1),
            |s| s,
        )
        .await?;

        let cmd = DeleteSchemaCommandBuilder::default()
            .schema_id(*schema.id())
            .build()
            .unwrap();

        state
            .command_bus()
            .execute::<_, DeleteSchemaCommandHandler>(&actor, &cmd)
            .await?;

        Ok(ApiResponse::OkEmpty(EmptyResource))
    }

    #[tracing::instrument]
    pub async fn find_schema(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        schema_path: Path<(Id, Id)>,
    ) -> ApiResult<SchemaResource> {
        Self::retrieve_schema(
            state.query_bus(),
            &actor,
            schema_path.0,
            schema_path.1,
            || ApiError::SchemaNotFound(schema_path.1),
            |schema| {
                ApiResponse::Created(
                    ResourceLink::Schema(schema_path.0, *schema.id()),
                    schema.into(),
                )
            },
        )
        .await
    }
}
