use std::marker::PhantomData;

use actix_web::{
    web::{Data, Json},
    HttpResponse, Responder,
};
use actix_web_validator::Path;

use super::{
    models::{ChannelCollectionPath, ChannelPath, CreateChannelRequest},
    service::ChannelRestService,
};

pub struct ChannelController<RS>(PhantomData<RS>);

impl<RS: ChannelRestService> ChannelController<RS> {
    pub async fn get(
        service: Data<RS>,
        path: Path<ChannelPath>,
    ) -> impl Responder {
        HttpResponse::Ok().finish()
    }

    pub async fn query(
        service: Data<RS>,
        path: Path<ChannelCollectionPath>,
    ) -> impl Responder {
        HttpResponse::Ok().finish()
    }

    pub async fn delete(
        service: Data<RS>,
        path: Path<ChannelPath>,
    ) -> impl Responder {
        HttpResponse::Ok().finish()
    }

    pub async fn update(
        service: Data<RS>,
        path: Path<ChannelPath>,
    ) -> impl Responder {
        HttpResponse::Ok().finish()
    }

    pub async fn create(
        service: Data<RS>,
        path: Path<ChannelCollectionPath>,
        payload: Json<CreateChannelRequest>,
    ) -> impl Responder {
        HttpResponse::Ok().finish()
    }
}
