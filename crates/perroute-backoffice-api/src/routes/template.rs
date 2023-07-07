use std::convert::identity;

use crate::{
    api::{
        models::template::{CreateTemplateRequest, UpdateTemplateRequest},
        response::{EmptyApiResult, NewApiResponse},
    },
    app::AppState,
    error::ApiError,
    extractors::actor::ActorExtractor,
};
use actix_web::{
    web::{Data, Json, Path},
    HttpResponse, Responder,
};
use perroute_commons::{new_id, prelude::Actor, types::id::Id};
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
    prelude::QueryBus,
};
use perroute_storage::models::template::Template;

use super::schema::SchemaRouter;

pub const TEMPLATES_RESOURCE_NAME: &str = "templates";
pub const TEMPLATE_RESOURCE_NAME: &str = "template";

pub struct TemplateRouter;

impl TemplateRouter {
    #[tracing::instrument]
    pub async fn query_templates(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        path: Path<(Id, Id, Id)>,
    ) -> impl Responder {
        let schema =
            SchemaRouter::retrieve_schema(state.query_bus(), &actor, *path.as_ref(), identity)
                .await
                .unwrap();

        HttpResponse::Ok().finish()
    }

    #[tracing::instrument]
    pub async fn create_template(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        path: Path<(Id, Id, Id)>,
        Json(body): Json<CreateTemplateRequest>,
    ) -> EmptyApiResult {
        let schema =
            SchemaRouter::retrieve_schema(state.query_bus(), &actor, *path.as_ref(), identity)
                .await
                .unwrap();

        let cmd = CreateTemplateCommandBuilder::default()
            .template_id(new_id!())
            .schema_id(body.schema_id)
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

        Ok(NewApiResponse::ok_empty())
    }

    #[tracing::instrument]
    pub async fn update_template(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        path: Path<(Id, Id, Id, Id)>,
        Json(body): Json<UpdateTemplateRequest>,
    ) -> EmptyApiResult {
        let template = Self::retrieve_template(state.query_bus(), &actor, *path.as_ref(), identity)
            .await
            .unwrap();

        let cmd = UpdateTemplateCommandBuilder::default()
            .template_id(*template.id())
            .html(body.html.map(Into::into))
            .text(body.text.map(Into::into))
            .subject(body.subject.map(Into::into))
            .build()
            .unwrap();
        let template = state
            .command_bus()
            .execute::<_, UpdateTemplateCommandHandler, _>(&actor, &cmd)
            .await?;

        Ok(NewApiResponse::ok_empty())
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
        let template = state
            .command_bus()
            .execute::<_, DeleteTemplateCommandHandler, _>(&actor, &cmd)
            .await?;

        Ok(NewApiResponse::ok_empty())
    }

    #[tracing::instrument]
    pub async fn find_template(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        path: Path<(Id, Id, Id, Id)>,
    ) -> EmptyApiResult {
        let template = Self::retrieve_template(state.query_bus(), &actor, *path.as_ref(), identity)
            .await
            .unwrap();

        Ok(NewApiResponse::ok_empty())
    }

    pub async fn retrieve_template<R>(
        query_bus: &QueryBus,
        actor: &Actor,
        path: (Id, Id, Id, Id),
        map: impl FnOnce(Template) -> R,
    ) -> Result<R, ApiError> {
        todo!()
    }
}
