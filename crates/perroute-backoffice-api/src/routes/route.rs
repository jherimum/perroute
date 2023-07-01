use actix_web::{HttpResponse, Responder};

pub const ROUTES_RESOURCE_NAME: &str = "routes";
pub const ROUTE_RESOURCE_NAME: &str = "route";

pub struct RouteRouter;

impl RouteRouter {
    #[tracing::instrument]
    pub async fn query() -> impl Responder {
        HttpResponse::Ok().finish()
    }

    #[tracing::instrument]
    pub async fn create() -> impl Responder {
        HttpResponse::Ok().finish()
    }

    #[tracing::instrument]
    pub async fn update() -> impl Responder {
        HttpResponse::Ok().finish()
    }

    #[tracing::instrument]
    pub async fn delete() -> impl Responder {
        HttpResponse::Ok().finish()
    }

    #[tracing::instrument]
    pub async fn find() -> impl Responder {
        HttpResponse::Ok().finish()
    }
}
