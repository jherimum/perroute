use super::message_type::MessageTypeRouter;
use crate::api::models::schema::UpdateSchemaRequest;
use crate::api::response::{
    ApiResponse, ApiResult, CollectionResourceModel, EmptyApiResult, SingleResourceModel,
};
use crate::error::ApiError;
use crate::links::ResourceLink;
use crate::{
    api::models::schema::{CreateSchemaRequest, SchemaResource},
    app::AppState,
    extractors::actor::ActorExtractor,
};
use actix_web::web::{Data, Json, Path};
use perroute_commons::new_id;
use perroute_commons::types::actor::Actor;
use perroute_commons::types::id::Id;
use perroute_commons::types::json_schema::JsonSchema;
use perroute_cqrs::command_bus::commands::{
    CreateSchemaCommandBuilder, DeleteSchemaCommandBuilder, UpdateSchemaCommandBuilder,
};
use perroute_cqrs::command_bus::handlers::schema::{
    create_schema::CreateSchemaCommandHandler, delete_schema::DeleteSchemaCommandHandler,
    update_schema::UpdateSchemaCommandHandler,
};
use perroute_cqrs::query_bus::bus::QueryBus;
use perroute_cqrs::query_bus::handlers::schema::find_schema::FindSchemaQueryHandler;
use perroute_cqrs::query_bus::handlers::schema::query_schemas::QuerySchemasQueryHandler;
use perroute_cqrs::query_bus::queries::{FindSchemaQueryBuilder, QuerySchemasQueryBuilder};
use perroute_storage::models::schema::Schema;
use std::convert::identity;
use tap::TapFallible;

pub const SCHEMAS_RESOURCE_NAME: &str = "schemas";
pub const SCHEMA_RESOURCE_NAME: &str = "schema";
pub const SCHEMA_CLONE_RESOURCE_NAME: &str = "schema_clone";

pub type SingleResult = ApiResult<SingleResourceModel<SchemaResource>>;
pub type CollectionResult = ApiResult<CollectionResourceModel<SchemaResource>>;

pub struct SchemaRouter;

impl SchemaRouter {
    pub async fn clone() -> String {
        todo!()
    }

    #[tracing::instrument(skip(state))]
    pub async fn query_schemas(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        path: Path<(Id, Id)>,
    ) -> CollectionResult {
        let message_type = MessageTypeRouter::retrieve_message_type(
            state.query_bus(),
            &actor,
            *path.as_ref(),
            identity,
        )
        .await?;

        let query = QuerySchemasQueryBuilder::default()
            .message_type_id(*message_type.id())
            .build()
            .unwrap();

        state
            .query_bus()
            .execute::<_, QuerySchemasQueryHandler, _>(&actor, &query)
            .await
            .map(|schemas| ApiResponse::ok((message_type, schemas)))
            .map_err(ApiError::from)
    }

    #[tracing::instrument(skip(state))]
    pub async fn create_schema(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        path: Path<(Id, Id)>,
        Json(body): Json<CreateSchemaRequest>,
    ) -> SingleResult {
        let message_type = MessageTypeRouter::retrieve_message_type(
            state.query_bus(),
            &actor,
            *path.as_ref(),
            identity,
        )
        .await?;

        let cmd = CreateSchemaCommandBuilder::default()
            .schema_id(new_id!())
            .message_type_id(*message_type.id())
            .schema(
                JsonSchema::try_from(body.schema)
                    .map_err(ApiError::from)
                    .tap_err(|e| tracing::error!("XXXXXXXXX:{e}"))?,
            )
            .build()
            .unwrap();

        Ok(state
            .command_bus()
            .execute::<_, CreateSchemaCommandHandler, _>(&actor, &cmd)
            .await
            .map(|schema| {
                ApiResponse::created(
                    ResourceLink::Schema(*schema.channel_id(), *message_type.id(), *schema.id()),
                    schema,
                )
            })?)
    }

    #[tracing::instrument(skip(state))]
    pub async fn update_schema(
        path: Path<(Id, Id, Id)>,
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        Json(body): Json<UpdateSchemaRequest>,
    ) -> SingleResult {
        let schema =
            Self::retrieve_schema(state.query_bus(), &actor, *path.as_ref(), identity).await?;

        let cmd = UpdateSchemaCommandBuilder::default()
            .schema_id(*schema.id())
            .schema(JsonSchema::try_from(body.schema).map_err(ApiError::from)?)
            .build()
            .unwrap();

        Ok(state
            .command_bus()
            .execute::<_, UpdateSchemaCommandHandler, _>(&actor, &cmd)
            .await
            .map(ApiResponse::ok)?)
    }

    #[tracing::instrument(skip(state))]
    pub async fn delete_schema(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        path: Path<(Id, Id, Id)>,
    ) -> EmptyApiResult {
        let schema =
            Self::retrieve_schema(state.query_bus(), &actor, *path.as_ref(), identity).await?;

        let cmd = DeleteSchemaCommandBuilder::default()
            .schema_id(*schema.id())
            .build()
            .unwrap();

        Ok(state
            .command_bus()
            .execute::<_, DeleteSchemaCommandHandler, _>(&actor, &cmd)
            .await
            .map(|_| ApiResponse::ok_empty())?)
    }

    #[tracing::instrument(skip(state))]
    pub async fn find_schema(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        path: Path<(Id, Id, Id)>,
    ) -> SingleResult {
        Self::retrieve_schema(state.query_bus(), &actor, *path.as_ref(), ApiResponse::ok).await
    }

    pub async fn retrieve_schema<R>(
        query_bus: &QueryBus,
        actor: &Actor,
        path: (Id, Id, Id),
        map: impl FnOnce(Schema) -> R,
    ) -> Result<R, ApiError> {
        let query = FindSchemaQueryBuilder::default()
            .schema_id(path.2)
            .message_type_id(Some(path.1))
            .channel_id(Some(path.0))
            .build()
            .unwrap();

        query_bus
            .execute::<_, FindSchemaQueryHandler, _>(actor, &query)
            .await
            .unwrap()
            .ok_or_else(|| ApiError::SchemaNotFound(path.1))
            .map(map)
    }
}
