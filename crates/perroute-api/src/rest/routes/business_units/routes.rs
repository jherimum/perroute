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

pub async fn get<RS: BusinessUnitRestService>(
    service: Data<RS>,
    path: Path<BusinessUnitPath>,
) -> ApiResult<ResourceModel<BusinessUnitModel>> {
    service
        .get(&path.into_inner().into())
        .await
        .map(ApiResponse::Ok)
}

pub async fn query<RS: BusinessUnitRestService>(
    service: Data<RS>,
) -> ApiResult<ResourceModelCollection<BusinessUnitModel>> {
    service.query().await.map(ApiResponse::Ok)
}

pub async fn delete<RS: BusinessUnitRestService>(
    service: Data<RS>,
    path: Path<BusinessUnitPath>,
) -> ApiResult<()> {
    service
        .delete(&path.into_inner().into())
        .await
        .map(ApiResponse::Ok)
}

pub async fn update<RS: BusinessUnitRestService>(
    service: Data<RS>,
    path: Path<BusinessUnitPath>,
    payload: Json<UpdateBusinessUnitRequest>,
) -> ApiResult<ResourceModel<BusinessUnitModel>> {
    service
        .update(&path.into_inner().into(), &payload)
        .await
        .map(ApiResponse::Ok)
}

pub async fn create<RS: BusinessUnitRestService>(
    service: Data<RS>,
    payload: Json<CreateBusinessUnitRequest>,
) -> ApiResult<()> {
    // service
    //     .create(&payload)
    //     .await
    //     .map(|id| ApiResponse::Created((), ()))
    todo!()
}
