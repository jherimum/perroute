use actix_web::{web::{Data, Json}, HttpResponse, Responder};
use actix_web_validator::Path;

use super::{models::{ChannelCollectionPath, ChannelPath, CreateChannelRequest}, service::ChannelRestService};

pub async fn get<RS: ChannelRestService>(service: Data<RS>,
    path: Path<ChannelPath>,) -> impl Responder {
    HttpResponse::Ok().finish()
}

pub async fn query<RS: ChannelRestService>(service: Data<RS>,
    path: Path<ChannelCollectionPath>,) -> impl Responder {
    HttpResponse::Ok().finish()
}

pub async fn delete<RS: ChannelRestService>(service: Data<RS>,
    path: Path<ChannelPath>,) -> impl Responder {
    HttpResponse::Ok().finish()
}

pub async fn update<RS: ChannelRestService>(service: Data<RS>,
    path: Path<ChannelPath>,) -> impl Responder {
    HttpResponse::Ok().finish()
}

pub async fn create<RS: ChannelRestService>(service: Data<RS>,
    path: Path<ChannelCollectionPath>, payload: Json<CreateChannelRequest>) -> impl Responder {
    HttpResponse::Ok().finish()
}
