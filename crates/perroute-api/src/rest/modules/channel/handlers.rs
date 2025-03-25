use actix_web::{
    web::{Data, Json},
    HttpResponse, Responder,
};
use actix_web_validator::Path;
use perroute_commons::types::actor::Actor;

use crate::rest::{
    models::{resource::ResourceModel, ApiResponse},
    modules::ApiResult,
};

use super::{
    models::{
        ChannelCollectionPath, ChannelModel, ChannelPath, CreateChannelRequest,
        UpdateChannelRequest,
    },
    service::ChannelRestService,
};

pub async fn get<RS: ChannelRestService>(
    service: Data<RS>,
    path: Path<ChannelPath>,
) -> impl Responder {
    HttpResponse::Ok().finish()
}

pub async fn query<RS: ChannelRestService>(
    service: Data<RS>,
    path: Path<ChannelCollectionPath>,
) -> impl Responder {
    HttpResponse::Ok().finish()
}

pub async fn delete<RS: ChannelRestService>(
    service: Data<RS>,
    path: Path<ChannelPath>,
) -> impl Responder {
    HttpResponse::Ok().finish()
}

pub async fn update<RS: ChannelRestService>(
    service: Data<RS>,
    path: Path<ChannelPath>,
    req: Json<UpdateChannelRequest>,
) -> ApiResult<ResourceModel<ChannelModel>> {
    let r = service.update(&Actor::System, &path, &req).await?;
    Ok(ApiResponse::ok(r))
}

pub async fn create<RS: ChannelRestService>(
    service: Data<RS>,
    path: Path<ChannelCollectionPath>,
    payload: Json<CreateChannelRequest>,
) -> ApiResult<ResourceModel<ChannelModel>> {
    let bu = service.create(&Actor::System, &path, &payload).await?;
    Ok(ApiResponse::created(bu))
}
