use super::{
    models::{
        CreateMessageTypeRequest, MessageTypeCollectionPath, MessageTypeModel,
        MessageTypePath, UpdateMessageTypeRequest,
    },
    service::MessageTypeRestService,
};
use crate::rest::{
    models::{
        resource::ResourceModel, resource::ResourceModelCollection, ApiResponse,
    },
    modules::ApiResult,
};
use actix_web::web::{Data, Json, Path};
use perroute_commons::types::actor::Actor;

pub async fn get<RS: MessageTypeRestService>(
    service: Data<RS>,
    path: Path<MessageTypePath>,
) -> ApiResult<ResourceModel<MessageTypeModel>> {
    service
        .get(&Actor::System, &path)
        .await
        .map(ApiResponse::ok)
}

pub async fn query<RS: MessageTypeRestService>(
    service: Data<RS>,
    path: Path<MessageTypeCollectionPath>,
) -> ApiResult<ResourceModelCollection<MessageTypeModel>> {
    service
        .query(&Actor::System, &path)
        .await
        .map(ApiResponse::ok)
}

pub async fn delete<RS: MessageTypeRestService>(
    service: Data<RS>,
    path: Path<MessageTypePath>,
) -> ApiResult<()> {
    service
        .delete(&Actor::System, &path)
        .await
        .map(|_| ApiResponse::no_content())
}

pub async fn update<RS: MessageTypeRestService>(
    service: Data<RS>,
    path: Path<MessageTypePath>,
    payload: Json<UpdateMessageTypeRequest>,
) -> ApiResult<ResourceModel<MessageTypeModel>> {
    service
        .update(&Actor::System, &path, &payload)
        .await
        .map(ApiResponse::ok)
}

pub async fn create<RS: MessageTypeRestService>(
    service: Data<RS>,
    path: Path<MessageTypeCollectionPath>,
    payload: Json<CreateMessageTypeRequest>,
) -> ApiResult<ResourceModel<MessageTypeModel>> {
    let mt = service.create(&Actor::System, &path, &payload).await?;
    Ok(ApiResponse::created(mt))
}
