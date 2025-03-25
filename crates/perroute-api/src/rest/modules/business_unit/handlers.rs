use actix_web::web::Data;
use actix_web_validator::{Json, Path};
use perroute_commons::types::actor::Actor;

use crate::rest::{
    models::{
        resource::{ResourceModel, ResourceModelCollection},
        ApiResponse,
    },
    modules::ApiResult,
};
use super::{
    models::{
        BusinessUnitCollectionPath, BusinessUnitModel, BusinessUnitPath,
        CreateBusinessUnitRequest, UpdateBusinessUnitRequest,
    },
    service::BusinessUnitRestService,
};

pub async fn get_business_unit<RS: BusinessUnitRestService + 'static>(
    service: Data<RS>,
    path: Path<BusinessUnitPath>,
) -> ApiResult<ResourceModel<BusinessUnitModel>> {
    service
        .get(&Actor::System, &path)
        .await
        .map(ApiResponse::ok)
}

pub async fn query_business_units<RS: BusinessUnitRestService + 'static>(
    service: Data<RS>,
    path: Path<BusinessUnitCollectionPath>,
) -> ApiResult<ResourceModelCollection<BusinessUnitModel>> {
    let bus = service.query(&Actor::System, &path).await?;
    Ok(ApiResponse::ok(bus))
}

pub async fn delete_business_unit<RS: BusinessUnitRestService + 'static>(
    service: Data<RS>,
    path: Path<BusinessUnitPath>,
) -> ApiResult<()> {
    service
        .delete(&Actor::System, &path)
        .await
        .map(|_| ApiResponse::no_content())
}

pub async fn update_business_unit<RS: BusinessUnitRestService + 'static>(
    service: Data<RS>,
    path: Path<BusinessUnitPath>,
    payload: Json<UpdateBusinessUnitRequest>,
) -> ApiResult<ResourceModel<BusinessUnitModel>> {
    service
        .update(&Actor::System, &path, &payload)
        .await
        .map(ApiResponse::ok)
}

pub async fn create_business_unit<RS: BusinessUnitRestService + 'static>(
    service: Data<RS>,
    path: Path<BusinessUnitCollectionPath>,
    payload: Json<CreateBusinessUnitRequest>,
) -> ApiResult<ResourceModel<BusinessUnitModel>> {
    let model = BusinessUnitRestService::create(
        service.get_ref(),
        &Actor::System,
        &path,
        &payload,
    )
    .await?;

    Ok(ApiResponse::created(model))
}
