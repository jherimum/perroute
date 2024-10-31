use super::{
    models::{
        CreateMessageTypeRequest, MessageTypeCollectionPath, MessageTypeModel, MessageTypePath,
        UpdateMessageTypeRequest,
    },
    service::MessageTypeRestService,
};
use crate::rest::{
    models::{ApiResponse, ResourceModel, ResourceModelCollection},
    routes::ApiResult,
};
use actix_web::web::{Data, Json, Path};
use perroute_commons::types::actor::Actor;
use url::Url;

pub async fn get<RS: MessageTypeRestService>(
    service: Data<RS>,
    path: Path<MessageTypePath>,
) -> ApiResult<ResourceModel<MessageTypeModel>> {
    service
        .get(&Actor::System, &path)
        .await
        .map(|b| ApiResponse::ok(b))
}

pub async fn query<RS: MessageTypeRestService>(
    service: Data<RS>,
    path: Path<MessageTypeCollectionPath>,
) -> ApiResult<ResourceModelCollection<MessageTypeModel>> {
    service
        .query(&Actor::System, &path)
        .await
        .map(|bus| ApiResponse::ok(bus))
}

pub async fn delete<RS: MessageTypeRestService>(
    service: Data<RS>,
    path: Path<MessageTypePath>,
) -> ApiResult<()> {
    service
        .delete(&Actor::System, &path)
        .await
        .map(|_| ApiResponse::ok_empty())
}

pub async fn update<RS: MessageTypeRestService>(
    service: Data<RS>,
    path: Path<MessageTypePath>,
    payload: Json<UpdateMessageTypeRequest>,
) -> ApiResult<ResourceModel<MessageTypeModel>> {
    service
        .update(&Actor::System, &path, &payload)
        .await
        .map(|_| ApiResponse::ok_empty())
}

pub async fn create<RS: MessageTypeRestService>(
    service: Data<RS>,
    path: Path<MessageTypeCollectionPath>,
    payload: Json<CreateMessageTypeRequest>,
) -> ApiResult<()> {
    service.create(&Actor::System, &path, &payload).await?;
    Ok(ApiResponse::created_empty(
        Url::parse("http://wine.com.br").unwrap(),
    ))
}
