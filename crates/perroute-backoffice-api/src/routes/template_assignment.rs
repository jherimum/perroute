use crate::{app::AppState, extractors::actor::ActorExtractor};
use actix_web::{web::Data, HttpResponse};

pub struct TemplateAssignmentRouter;

impl TemplateAssignmentRouter {
    pub const SINGLE_RESOURCE_NAME: &str = "template-assignment";
    pub const COLLECTION_RESOURCE_NAME: &str = "template-assignments";

    #[tracing::instrument]
    pub async fn create(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
    ) -> HttpResponse {
        HttpResponse::Ok().finish()
    }

    #[tracing::instrument]
    pub async fn update(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
    ) -> HttpResponse {
        HttpResponse::Ok().finish()
    }

    #[tracing::instrument]
    pub async fn delete(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
    ) -> HttpResponse {
        HttpResponse::Ok().finish()
    }

    #[tracing::instrument]
    pub async fn query(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
    ) -> HttpResponse {
        HttpResponse::Ok().finish()
    }

    #[tracing::instrument]
    pub async fn find(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
    ) -> HttpResponse {
        HttpResponse::Ok().finish()
    }
}
