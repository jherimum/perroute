use actix_web::{HttpResponse, Responder};

pub const ROUTES_RESOURCE_NAME: &str = "routes";
pub const ROUTE_RESOURCE_NAME: &str = "route";

pub struct RouteRouter;

impl RouteRouter {
    #[tracing::instrument]
    pub async fn query_routes() -> impl Responder {
        HttpResponse::Ok().finish()
    }

    #[tracing::instrument]
    pub async fn create_route() -> impl Responder {
        HttpResponse::Ok().finish()
    }

    #[tracing::instrument]
    pub async fn update_route() -> impl Responder {
        HttpResponse::Ok().finish()
    }

    #[tracing::instrument]
    pub async fn delete_route() -> impl Responder {
        HttpResponse::Ok().finish()
    }

    #[tracing::instrument]
    pub async fn find_route() -> impl Responder {
        HttpResponse::Ok().finish()
    }
}
