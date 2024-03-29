use crate::{app::AppState, extractors::actor::ActorExtractor};
use actix_web::{
    web::{Data, Path},
    HttpResponse, Responder,
};
use perroute_commons::types::id::Id;

pub struct RouteRouter;

impl RouteRouter {
    pub const ROUTES_RESOURCE_NAME: &str = "routes";
    pub const ROUTE_RESOURCE_NAME: &str = "route";

    #[tracing::instrument]
    pub async fn query_routes(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
    ) -> impl Responder {
        HttpResponse::Ok().finish()
    }

    #[tracing::instrument]
    pub async fn create_route(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
    ) -> impl Responder {
        HttpResponse::Ok().finish()
    }

    #[tracing::instrument]
    pub async fn update_route(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        route: Path<Id>,
    ) -> impl Responder {
        HttpResponse::Ok().finish()
    }

    #[tracing::instrument]
    pub async fn delete_route(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        route: Path<Id>,
    ) -> impl Responder {
        HttpResponse::Ok().finish()
    }

    #[tracing::instrument]
    pub async fn find_route(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        route: Path<Id>,
    ) -> impl Responder {
        HttpResponse::Ok().finish()
    }
}
