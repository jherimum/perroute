use super::{
    models::{
        BusinessUnitCollectionPath, BusinessUnitModel, BusinessUnitPath, CreateBusinessUnitRequest,
        UpdateBusinessUnitRequest,
    },
    service::BusinessUnitRestService,
};
use crate::rest::{
    models::{ApiResponse, ResourceModel, ResourceModelCollection},
    modules::ApiResult,
};
use actix_web::web::{Data, Path};
use actix_web_validator::Json;
use perroute_commons::types::actor::Actor;
use std::marker::PhantomData;
use url::Url;

pub struct BusinessUnitController<RS>(PhantomData<RS>);

impl<RS: BusinessUnitRestService> BusinessUnitController<RS> {
    pub async fn get(
        service: Data<RS>,
        path: Path<BusinessUnitPath>,
    ) -> ApiResult<ResourceModel<BusinessUnitModel>> {
        service
            .get(&Actor::System, &path)
            .await
            .map(ApiResponse::ok)
    }

    pub async fn query(
        service: Data<RS>,
        path: Path<BusinessUnitCollectionPath>,
    ) -> ApiResult<ResourceModelCollection<BusinessUnitModel>> {
        service
            .query(&Actor::System, &path)
            .await
            .map(ApiResponse::ok)
    }

    pub async fn delete(service: Data<RS>, path: Path<BusinessUnitPath>) -> ApiResult<()> {
        service
            .delete(&Actor::System, &path)
            .await
            .map(|_| ApiResponse::ok_empty())
    }

    pub async fn update(
        service: Data<RS>,
        path: Path<BusinessUnitPath>,
        payload: Json<UpdateBusinessUnitRequest>,
    ) -> ApiResult<ResourceModel<BusinessUnitModel>> {
        service
            .update(&Actor::System, &path, &payload)
            .await
            .map(|_| ApiResponse::ok_empty())
    }

    pub async fn create(
        service: Data<RS>,
        path: Path<BusinessUnitCollectionPath>,
        payload: Json<CreateBusinessUnitRequest>,
    ) -> ApiResult<()> {
        service.create(&Actor::System, &path, &payload).await?;
        Ok(ApiResponse::created_empty(
            Url::parse("http://wine.com.br").unwrap(),
        ))
    }
}
