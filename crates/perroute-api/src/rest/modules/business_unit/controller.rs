use super::{
    models::{BusinessUnitModel, CreateBusinessUnitRequest, UpdateBusinessUnitRequest},
    service::BusinessUnitRestService,
    BUSINESS_UNIT_COLLECTION_RESOURCE_NAME, BUSINESS_UNIT_RESOURCE_NAME,
};
use crate::rest::{
    models::{
        link::ResourcePath,
        resource::{ResourceModel, ResourceModelCollection},
        ApiResponse,
    },
    modules::ApiResult,
};
use actix_web::web::{Data, Path};
use actix_web_validator::Json;
use perroute_commons::types::{actor::Actor, id::Id};
use serde::Deserialize;
use std::marker::PhantomData;
use url::Url;

#[derive(Debug, Deserialize)]
pub struct BusinessUnitPath(String);

impl BusinessUnitPath {
    pub fn new(id: &str) -> Self {
        BusinessUnitPath(id.to_string())
    }

    pub fn id(&self) -> Id {
        Id::from(&self.0)
    }
}

impl ResourcePath for BusinessUnitPath {
    fn url(&self, req: &actix_web::HttpRequest) -> Url {
        req.url_for(BUSINESS_UNIT_RESOURCE_NAME, [&self.0]).unwrap()
    }
}

#[derive(Debug, Deserialize)]
pub struct BusinessUnitCollectionPath;

impl ResourcePath for BusinessUnitCollectionPath {
    fn url(&self, req: &actix_web::HttpRequest) -> Url {
        req.url_for_static(BUSINESS_UNIT_COLLECTION_RESOURCE_NAME)
            .unwrap()
    }
}

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
            .map(ApiResponse::ok)
    }

    pub async fn create(
        service: Data<RS>,
        path: Path<BusinessUnitCollectionPath>,
        payload: Json<CreateBusinessUnitRequest>,
    ) -> ApiResult<ResourceModel<BusinessUnitModel>> {
        let bu = service.create(&Actor::System, &path, &payload).await?;
        Ok(ApiResponse::created(
            Url::parse("http://wine.com.br").unwrap(),
            bu,
        ))
    }
}
