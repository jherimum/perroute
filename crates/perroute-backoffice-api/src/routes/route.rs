use crate::{app::AppState, extractors::actor::ActorExtractor};
use actix_web::{
    web::{Data, Path},
    HttpResponse, Responder,
};
use perroute_commons::types::id::Id;

pub const ROUTES_RESOURCE_NAME: &str = "routes";
pub const ROUTE_RESOURCE_NAME: &str = "route";

pub struct RouteRouter;

impl RouteRouter {
    #[tracing::instrument]
    pub async fn query_routes(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        routes: Path<Id>,
    ) -> impl Responder {
        HttpResponse::Ok().finish()
    }

    #[tracing::instrument]
    pub async fn create_route(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        routes: Path<Id>,
    ) -> impl Responder {
        HttpResponse::Ok().finish()
    }

    #[tracing::instrument]
    pub async fn update_route(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        route: Path<(Id, Id)>,
    ) -> impl Responder {
        HttpResponse::Ok().finish()
    }

    #[tracing::instrument]
    pub async fn delete_route(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        route: Path<(Id, Id)>,
    ) -> impl Responder {
        HttpResponse::Ok().finish()
    }

    #[tracing::instrument]
    pub async fn find_route(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        route: Path<(Id, Id)>,
    ) -> impl Responder {
        HttpResponse::Ok().finish()
    }
}
