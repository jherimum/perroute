use super::models::{
    BusinessUnitModel, BusinessUnitPath, CreateBusinessUnitRequest, UpdateBusinessUnitRequest,
};
use crate::rest::{
    models::{ApiResponse, ResourceModel, ResourceModelCollection},
    routes::ApiResult,
    services::business_units::BusinessUnitRestService,
};
use actix_web::web::{Data, Path};
use actix_web_validator::Json;
use perroute_commons::types::actor::Actor;
use url::Url;

pub async fn get<RS: BusinessUnitRestService>(
    service: Data<RS>,
    path: Path<BusinessUnitPath>,
) -> ApiResult<ResourceModel<BusinessUnitModel>> {
    service
        .get(&Actor::System, &path)
        .await
        .map(|b| ApiResponse::ok(b))
}

pub async fn query<RS: BusinessUnitRestService>(
    service: Data<RS>,
) -> ApiResult<ResourceModelCollection<BusinessUnitModel>> {
    service
        .query(&Actor::System)
        .await
        .map(|bus| ApiResponse::ok(bus))
}

pub async fn delete<RS: BusinessUnitRestService>(
    service: Data<RS>,
    path: Path<BusinessUnitPath>,
) -> ApiResult<()> {
    service
        .delete(&Actor::System, &path)
        .await
        .map(|_| ApiResponse::ok_empty())
}

pub async fn update<RS: BusinessUnitRestService>(
    service: Data<RS>,
    path: Path<BusinessUnitPath>,
    payload: Json<UpdateBusinessUnitRequest>,
) -> ApiResult<ResourceModel<BusinessUnitModel>> {
    service
        .update(&Actor::System, &path, &payload)
        .await
        .map(|_| ApiResponse::ok_empty())
}

pub async fn create<RS: BusinessUnitRestService>(
    service: Data<RS>,
    payload: Json<CreateBusinessUnitRequest>,
) -> ApiResult<()> {
    service.create(&Actor::System, &payload).await?;
    Ok(ApiResponse::created_empty(
        Url::parse("http://wine.com.br").unwrap(),
    ))
}
