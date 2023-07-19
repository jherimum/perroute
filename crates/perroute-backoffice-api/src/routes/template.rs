use std::convert::identity;

use super::schema::SchemaRouter;
use crate::{
    api::{
        models::template::{CreateTemplateRequest, TemplateResource, UpdateTemplateRequest},
        response::{ApiResponse, ApiResult, EmptyApiResult, ResourceModel},
    },
    app::AppState,
    error::ApiError,
    extractors::actor::ActorExtractor,
    links::ResourceLink,
};
use actix_web::web::{Data, Json, Path};
use perroute_commons::{
    new_id,
    types::{actor::Actor, id::Id},
};
use perroute_cqrs::{
    command_bus::{
        commands::{
            CreateTemplateCommandBuilder, DeleteTemplateCommandBuilder,
            UpdateTemplateCommandBuilder,
        },
        handlers::template::{
            create_template::CreateTemplateCommandHandler,
            delete_template::DeleteTemplateCommandHandler,
            update_template::UpdateTemplateCommandHandler,
        },
    },
    query_bus::{
        bus::QueryBus,
        handlers::template::{
            find_tempate::FindTemplateQueryHandler, query_templates::QueryTemplatesQueryHandler,
        },
        queries::{FindTemplateQueryBuilder, QueryTemplatesQueryBuilder},
    },
};
use perroute_storage::models::template::Template;

pub type SingleResult = ApiResult<ResourceModel<TemplateResource>>;
pub type CollectionResult = ApiResult<ResourceModel<Vec<ResourceModel<TemplateResource>>>>;

pub struct TemplateRouter;

impl TemplateRouter {
    pub const TEMPLATES_RESOURCE_NAME: &str = "templates";
    pub const TEMPLATE_RESOURCE_NAME: &str = "template";

    #[tracing::instrument]
    pub async fn query_templates(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        path: Path<(Id, Id, Id)>,
    ) -> CollectionResult {
        let schema =
            SchemaRouter::retrieve_schema(state.query_bus(), &actor, *path.as_ref(), identity)
                .await
                .unwrap();

        let query = QueryTemplatesQueryBuilder::default()
            .schema_id(Some(*schema.id()))
            .build()
            .unwrap();

        state
            .query_bus()
            .execute::<_, QueryTemplatesQueryHandler, _>(&actor, &query)
            .await
            .map(|templates| ApiResponse::ok((schema, templates)))
            .map_err(ApiError::from)
    }

    #[tracing::instrument]
    pub async fn create_template(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        path: Path<(Id, Id, Id)>,
        Json(body): Json<CreateTemplateRequest>,
    ) -> SingleResult {
        let schema =
            SchemaRouter::retrieve_schema(state.query_bus(), &actor, *path.as_ref(), identity)
                .await
                .unwrap();

        let cmd = CreateTemplateCommandBuilder::default()
            .template_id(new_id!())
            .schema_id(*schema.id())
            .name(body.name)
            .html(body.html.map(Into::into))
            .text(body.text.map(Into::into))
            .subject(body.subject.map(Into::into))
            .build()
            .unwrap();
        let template = state
            .command_bus()
            .execute::<_, CreateTemplateCommandHandler, _>(&actor, &cmd)
            .await?;

        Ok(ApiResponse::created(
            ResourceLink::Template(path.0, path.1, path.2, *template.id()),
            template,
        ))
    }

    #[tracing::instrument]
    pub async fn update_template(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        path: Path<(Id, Id, Id, Id)>,
        Json(body): Json<UpdateTemplateRequest>,
    ) -> SingleResult {
        let template = Self::retrieve_template(state.query_bus(), &actor, *path.as_ref(), identity)
            .await
            .unwrap();

        let cmd = UpdateTemplateCommandBuilder::default()
            .template_id(*template.id())
            .html(body.html.map(Into::into))
            .text(body.text.map(Into::into))
            .subject(body.subject.map(Into::into))
            .name(body.name)
            .build()
            .unwrap();

        state
            .command_bus()
            .execute::<_, UpdateTemplateCommandHandler, _>(&actor, &cmd)
            .await
            .map_err(ApiError::from)
            .map(ApiResponse::ok)
    }

    #[tracing::instrument]
    pub async fn delete_template(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        path: Path<(Id, Id, Id, Id)>,
    ) -> EmptyApiResult {
        let template = Self::retrieve_template(state.query_bus(), &actor, *path.as_ref(), identity)
            .await
            .unwrap();
        let cmd = DeleteTemplateCommandBuilder::default()
            .template_id(*template.id())
            .build()
            .unwrap();
        state
            .command_bus()
            .execute::<_, DeleteTemplateCommandHandler, _>(&actor, &cmd)
            .await?;

        Ok(ApiResponse::ok_empty())
    }

    #[tracing::instrument]
    pub async fn find_template(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        path: Path<(Id, Id, Id, Id)>,
    ) -> SingleResult {
        Self::retrieve_template(state.query_bus(), &actor, *path.as_ref(), ApiResponse::ok).await
    }

    pub async fn retrieve_template<R>(
        query_bus: &QueryBus,
        actor: &Actor,
        path: (Id, Id, Id, Id),
        map: impl FnOnce(Template) -> R + Send + Sync,
    ) -> Result<R, ApiError> {
        let query = FindTemplateQueryBuilder::default()
            .template_id(path.3)
            .schema_id(Some(path.2))
            .message_type_id(Some(path.1))
            .channel_id(Some(path.0))
            .build()
            .unwrap();

        query_bus
            .execute::<_, FindTemplateQueryHandler, _>(actor, &query)
            .await?
            .ok_or_else(|| ApiError::TemplateNotFound(path.3))
            .map(map)
    }
}
