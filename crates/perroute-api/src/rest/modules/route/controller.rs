use std::marker::PhantomData;

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

pub struct RouteController<RS>(PhantomData<RS>);

impl<RS: RouteRestService> RouteController<RS> {
    pub async fn get(
        service: Data<RS>,
        path: Path<RoutePath>,
    ) -> impl Responder {
        HttpResponse::Ok().finish()
    }

    pub async fn query(
        service: Data<RS>,
        path: Path<RouteCollectionPath>,
    ) -> impl Responder {
        HttpResponse::Ok().finish()
    }

    pub async fn delete(
        service: Data<RS>,
        path: Path<RoutePath>,
    ) -> impl Responder {
        HttpResponse::Ok().finish()
    }

    pub async fn update(
        service: Data<RS>,
        path: Path<RoutePath>,
        payload: Json<UpdateRouteRequest>,
    ) -> impl Responder {
        HttpResponse::Ok().finish()
    }

    pub async fn create(
        service: Data<RS>,
        path: Path<RouteCollectionPath>,
        payload: Json<CreateRouteRequest>,
    ) -> impl Responder {
        HttpResponse::Ok().finish()
    }
}
