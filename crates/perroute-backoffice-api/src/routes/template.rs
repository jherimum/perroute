use crate::{
    api::{
        models::template::{CreateTemplateRequest, TemplateResource, UpdateTemplateRequest},
        response::{
            ApiResponse, ApiResult, CollectionResourceModel, EmptyApiResult, SingleResourceModel,
        },
    },
    app::AppState,
    error::ApiError,
    extractors::actor::ActorExtractor,
    links::ResourceLink,
};
use actix_web::web::{Data, Path};
use actix_web_validator::Json;
use perroute_commons::types::{actor::Actor, id::Id};
use perroute_cqrs::query_bus::{
    bus::QueryBus,
    handlers::template::{
        find_tempate::{FindTemplateQueryBuilder, FindTemplateQueryHandler},
        query_templates::{QueryTemplatesQueryBuilder, QueryTemplatesQueryHandler},
    },
};

use perroute_commandbus::command::template::{
    create_template::{CreateTemplateCommand, CreateTemplateCommandBuilder},
    delete_template::{DeleteTemplateCommand, DeleteTemplateCommandBuilder},
    update_template::{UpdateTemplateCommand, UpdateTemplateCommandBuilder},
};
use perroute_storage::models::template::Template;
use std::convert::identity;

pub type SingleResult = ApiResult<SingleResourceModel<TemplateResource>>;
pub type CollectionResult = ApiResult<CollectionResourceModel<TemplateResource>>;

pub struct TemplateRouter;

impl TemplateRouter {
    pub const TEMPLATES_RESOURCE_NAME: &str = "templates";
    pub const TEMPLATE_RESOURCE_NAME: &str = "template";

    #[tracing::instrument(skip(state))]
    pub async fn query_templates(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
    ) -> CollectionResult {
        let query = QueryTemplatesQueryBuilder::default().build().unwrap();

        Ok(state
            .query_bus()
            .execute::<_, QueryTemplatesQueryHandler, _>(&actor, &query)
            .await
            .map(ApiResponse::ok)?)
    }

    #[tracing::instrument(skip(state))]
    pub async fn create_template(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        Json(body): Json<CreateTemplateRequest>,
    ) -> SingleResult {
        let cmd = CreateTemplateCommandBuilder::default()
            .id(Id::new())
            .name(body.name()?)
            .subject(body.subject()?)
            .html(body.html()?)
            .text(body.text()?)
            .build()
            .unwrap();
        let template = state
            .command_bus()
            .execute::<CreateTemplateCommand, _>(actor, cmd)
            .await?;

        Ok(ApiResponse::created(
            ResourceLink::Template(*template.id()),
            template,
        ))
    }

    #[tracing::instrument(skip(state))]
    pub async fn update_template(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        path: Path<Id>,
        Json(body): Json<UpdateTemplateRequest>,
    ) -> SingleResult {
        let cmd = UpdateTemplateCommandBuilder::default()
            .id(path.into_inner())
            .name(body.name()?)
            .html(body.html()?)
            .text(body.text()?)
            .subject(body.subject()?)
            .build()
            .unwrap();

        state
            .command_bus()
            .execute::<UpdateTemplateCommand, _>(actor, cmd)
            .await
            .map_err(ApiError::from)
            .map(ApiResponse::ok)
    }

    #[tracing::instrument(skip(state))]
    pub async fn delete_template(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        path: Path<Id>,
    ) -> EmptyApiResult {
        let template = Self::retrieve_template(state.query_bus(), &actor, *path.as_ref(), identity)
            .await
            .unwrap();
        let cmd = DeleteTemplateCommandBuilder::default()
            .id(*template.id())
            .build()
            .unwrap();
        state
            .command_bus()
            .execute::<DeleteTemplateCommand, _>(actor, cmd)
            .await?;

        Ok(ApiResponse::ok_empty())
    }

    #[tracing::instrument(skip(state))]
    pub async fn find_template(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        path: Path<Id>,
    ) -> SingleResult {
        Self::retrieve_template(state.query_bus(), &actor, *path.as_ref(), ApiResponse::ok).await
    }

    pub async fn retrieve_template<R>(
        query_bus: &QueryBus,
        actor: &Actor,
        path: Id,
        map: impl FnOnce(Template) -> R + Send + Sync,
    ) -> Result<R, ApiError> {
        let query = FindTemplateQueryBuilder::default()
            .template_id(path)
            .build()
            .unwrap();

        Ok(query_bus
            .execute::<_, FindTemplateQueryHandler, _>(actor, &query)
            .await
            .map(map)?)
    }
}
