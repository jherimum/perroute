use crate::{
    api::{
        models::template::{CreateTemplateRequest, UpdateTemplateRequest},
        response::{EmptyApiResult, NewApiResponse},
    },
    app::AppState,
    extractors::actor::ActorExtractor,
};
use actix_web::{
    web::{Data, Json, Path},
    HttpResponse, Responder,
};
use perroute_commons::{new_id, types::id::Id};
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
    query_bus::queries::QueryTemplatesQueryBuilder,
};

pub const TEMPLATES_RESOURCE_NAME: &str = "templates";
pub const TEMPLATE_RESOURCE_NAME: &str = "template";

pub struct TemplateRouter;

impl TemplateRouter {
    #[tracing::instrument]
    pub async fn query_templates(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
    ) -> impl Responder {
        let query = QueryTemplatesQueryBuilder::default().build().unwrap();
        //state.query_bus().execute::<QueryTem>(&actor, &query);

        HttpResponse::Ok().finish()
    }

    #[tracing::instrument]
    pub async fn create_template(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        Json(body): Json<CreateTemplateRequest>,
    ) -> EmptyApiResult {
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
        template_path: Path<Id>,
        Json(body): Json<UpdateTemplateRequest>,
    ) -> EmptyApiResult {
        let cmd = UpdateTemplateCommandBuilder::default()
            .template_id(template_path.into_inner())
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
        template_path: Path<Id>,
    ) -> EmptyApiResult {
        let cmd = DeleteTemplateCommandBuilder::default()
            .template_id(template_path.into_inner())
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
        template_path: Path<(Id, Id)>,
    ) -> EmptyApiResult {
        Ok(NewApiResponse::ok_empty())
    }
}
