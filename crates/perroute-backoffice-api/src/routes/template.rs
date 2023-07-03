use actix_web::{HttpResponse, Responder};

pub const TEMPLATES_RESOURCE_NAME: &str = "templates";
pub const TEMPLATE_RESOURCE_NAME: &str = "template";

pub struct TemplateRouter;

impl TemplateRouter {
    #[tracing::instrument]
    pub async fn query_templates() -> impl Responder {
        HttpResponse::Ok().finish()
    }

    #[tracing::instrument]
    pub async fn create_template() -> impl Responder {
        HttpResponse::Ok().finish()
    }

    #[tracing::instrument]
    pub async fn update_template() -> impl Responder {
        HttpResponse::Ok().finish()
    }

    #[tracing::instrument]
    pub async fn delete_template() -> impl Responder {
        HttpResponse::Ok().finish()
    }

    #[tracing::instrument]
    pub async fn find_template() -> impl Responder {
        HttpResponse::Ok().finish()
    }
}
