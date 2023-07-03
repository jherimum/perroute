use actix_web::{
    web::{Data, Json, Path},
    HttpResponse, Responder,
};
use perroute_commons::{
    new_id,
    types::{id::Id, json_schema::JsonSchema},
};
use perroute_cqrs::{
    command_bus::{
        commands::{CreateSchemaCommandBuilder, UpdateSchemaCommand, UpdateSchemaCommandBuilder},
        handlers::schema::{
            create_schema::CreateSchemaCommandHandler, update_schema::UpdateSchemaCommandHandler,
        },
    },
    query_bus::{
        handlers::schema::{
            find_channel_schema::FindChannelSchemaQueryHandler,
            find_schema_by_id::FindSchemaByIdQueryHandler,
        },
        queries::{FindChannelSchemaQueryBuilder, FindSchemaByIdQueryBuilder},
    },
};

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

pub const SCHEMAS_RESOURCE_NAME: &str = "schemas";
pub const SCHEMA_RESOURCE_NAME: &str = "schema";

pub struct SchemaRouter;

impl SchemaRouter {
    #[tracing::instrument]
    pub async fn query_schemas(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        schemas_path: Path<Id>,
    ) -> ApiResult<EmptyResource> {
        let channel = ChannelRouter::retrieve_channel(
            state.query_bus(),
            &actor,
            &schemas_path.clone(),
            || ApiError::ChannelNotFound(schemas_path.into_inner()),
        )
        .await?;

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
        let channel = ChannelRouter::retrieve_channel(
            state.query_bus(),
            &actor,
            &schemas_path.clone(),
            || ApiError::ChannelNotFound(schemas_path.into_inner()),
        )
        .await?;

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

        let query = FindSchemaByIdQueryBuilder::default()
            .id(*cmd.schema_id())
            .build()
            .unwrap();

        let schema = state
            .query_bus()
            .execute::<_, FindSchemaByIdQueryHandler, _>(&actor, &query)
            .await?
            .unwrap();

        Ok(ApiResponse::Created(
            ResourceLink::Schema(*channel.id(), *schema.id()),
            schema.into(),
        ))
    }

    #[tracing::instrument]
    pub async fn update_schema(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        schema_path: Path<(Id, Id)>,
    ) -> impl Responder {
        let query = FindChannelSchemaQueryBuilder::default()
            .channel_id(schema_path.0)
            .schema_id(schema_path.1)
            .build()
            .unwrap();

        let schema = state
            .query_bus()
            .execute::<_, FindChannelSchemaQueryHandler, _>(&actor, &query)
            .await
            .unwrap()
            .unwrap();

        let cmd = UpdateSchemaCommandBuilder::default().build().unwrap();

        state
            .command_bus()
            .execute::<_, UpdateSchemaCommandHandler>(&actor, &cmd)
            .await
            .unwrap();

        HttpResponse::Ok().finish()
    }

    #[tracing::instrument]
    pub async fn delete_schema(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        schema_path: Path<(Id, Id)>,
    ) -> impl Responder {
        HttpResponse::Ok().finish()
    }

    #[tracing::instrument]
    pub async fn find_schema(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        schema_path: Path<(Id, Id)>,
    ) -> impl Responder {
        HttpResponse::Ok().finish()
    }
}
