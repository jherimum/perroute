use super::{
    models::{
        CreateRouteRequest, RouteCollectionPath, RoutePath, UpdateRouteRequest,
    },
    service::RouteRestService,
};
use actix_web::{
    web::{Data, Json},
    HttpResponse, Responder,
};
use actix_web_validator::Path;

pub async fn get<RS: RouteRestService>(
    service: Data<RS>,
    path: Path<RoutePath>,
) -> impl Responder {
    HttpResponse::Ok().finish()
}

pub async fn query<RS: RouteRestService>(
    service: Data<RS>,
    path: Path<RouteCollectionPath>,
) -> impl Responder {
    HttpResponse::Ok().finish()
}

pub async fn delete<RS: RouteRestService>(
    service: Data<RS>,
    path: Path<RoutePath>,
) -> impl Responder {
    HttpResponse::Ok().finish()
}

pub async fn update<RS: RouteRestService>(
    service: Data<RS>,
    path: Path<RoutePath>,
    payload: Json<UpdateRouteRequest>,
) -> impl Responder {
    HttpResponse::Ok().finish()
}

pub async fn create<RS: RouteRestService>(
    service: Data<RS>,
    path: Path<RouteCollectionPath>,
    payload: Json<CreateRouteRequest>,
) -> impl Responder {
    HttpResponse::Ok().finish()
}
