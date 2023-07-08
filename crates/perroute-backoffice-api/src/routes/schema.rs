use super::message_type::MessageTypeRouter;
use super::prelude::*;
use crate::api::models::schema::UpdateSchemaRequest;
use crate::links::ResourceLink;
use crate::{
    api::models::schema::{CreateSchemaRequest, SchemaResource},
    app::AppState,
    extractors::actor::ActorExtractor,
};
use perroute_cqrs::command_bus::handlers::schema::{
    create_schema::CreateSchemaCommandHandler, delete_schema::DeleteSchemaCommandHandler,
    update_schema::UpdateSchemaCommandHandler,
};
use perroute_cqrs::query_bus::handlers::schema::find_message_schema::FindMessageTypeSchemaQueryHandler;
use perroute_cqrs::query_bus::handlers::schema::query_message_type_schemas::QueryMessageTypeSchemasQueryHandler;
use perroute_storage::models::schema::Schema;
use std::convert::identity;

pub const SCHEMAS_RESOURCE_NAME: &str = "schemas";
pub const SCHEMA_RESOURCE_NAME: &str = "schema";

pub type SingleResult = ApiResult<SingleResourceModel<SchemaResource>>;
pub type CollectionResult = ApiResult<CollectionResourceModel<SchemaResource>>;

pub struct SchemaRouter;

impl SchemaRouter {
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

        let query = QueryMessageTypeSchemasQueryBuilder::default()
            .message_type_id(*message_type.id())
            .build()
            .unwrap();

        state
            .query_bus()
            .execute::<_, QueryMessageTypeSchemasQueryHandler, _>(&actor, &query)
            .await
            .map(|schemas| NewApiResponse::ok((message_type, schemas)))
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
                NewApiResponse::created(
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
            .map(NewApiResponse::ok)?)
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
            .map(|_| NewApiResponse::ok_empty())?)
    }

    #[tracing::instrument(skip(state))]
    pub async fn find_schema(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        path: Path<(Id, Id, Id)>,
    ) -> SingleResult {
        Self::retrieve_schema(
            state.query_bus(),
            &actor,
            *path.as_ref(),
            NewApiResponse::ok,
        )
        .await
    }

    pub async fn retrieve_schema<R>(
        query_bus: &QueryBus,
        actor: &Actor,
        path: (Id, Id, Id),
        map: impl FnOnce(Schema) -> R,
    ) -> Result<R, ApiError> {
        let query = FindMessageTypeSchemaQueryBuilder::default()
            .schema_id(path.2)
            .message_type_id(Some(path.1))
            .channel_id(Some(path.0))
            .build()
            .unwrap();

        query_bus
            .execute::<_, FindMessageTypeSchemaQueryHandler, _>(actor, &query)
            .await
            .unwrap()
            .ok_or_else(|| ApiError::SchemaNotFound(path.1))
            .map(map)
    }
}
