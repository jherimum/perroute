use crate::{app::AppState, extractors::actor::ActorExtractor};
use actix_web::{
    web::{Data, Path},
    HttpResponse, Responder,
};
use perroute_commons::types::id::Id;

pub const TEMPLATES_RESOURCE_NAME: &str = "templates";
pub const TEMPLATE_RESOURCE_NAME: &str = "template";

pub struct TemplateRouter;

impl TemplateRouter {
    #[tracing::instrument]
    pub async fn query_templates(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
    ) -> impl Responder {
        HttpResponse::Ok().finish()
    }

    #[tracing::instrument]
    pub async fn create_template(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        channel_path: Path<Id>,
    ) -> impl Responder {
        HttpResponse::Ok().finish()
    }

    #[tracing::instrument]
    pub async fn update_template(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        template_path: Path<(Id, Id)>,
    ) -> impl Responder {
        HttpResponse::Ok().finish()
    }

    #[tracing::instrument]
    pub async fn delete_template(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        template_path: Path<(Id, Id)>,
    ) -> impl Responder {
        HttpResponse::Ok().finish()
    }

    #[tracing::instrument]
    pub async fn find_template(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        template_path: Path<(Id, Id)>,
    ) -> impl Responder {
        HttpResponse::Ok().finish()
    }
}
