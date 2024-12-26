use super::{
    models::{
        BusinessUnitModel, CreateBusinessUnitRequest, UpdateBusinessUnitRequest,
    },
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
use actix_web::web::Data;
use actix_web_validator::{Json, Path};
use perroute_commons::types::{actor::Actor, id::Id};
use serde::Deserialize;
use std::marker::PhantomData;
use url::Url;
use validator::Validate;

#[derive(Debug, Deserialize, Clone, Validate, PartialEq, Eq)]
pub struct BusinessUnitPath {
    business_unit_id: String,
}

impl BusinessUnitPath {
    pub fn new(id: &str) -> Self {
        BusinessUnitPath {
            business_unit_id: id.to_string(),
        }
    }

    pub fn id(&self) -> Id {
        Id::from(&self.business_unit_id)
    }
}

impl ResourcePath for BusinessUnitPath {
    fn url(&self, req: &actix_web::HttpRequest) -> Url {
        req.url_for(BUSINESS_UNIT_RESOURCE_NAME, [&self.business_unit_id])
            .unwrap()
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct BusinessUnitCollectionPath;

impl Validate for BusinessUnitCollectionPath {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        Ok(())
    }
}

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
        let bus = service.query(&Actor::System, &path).await?;
        Ok(ApiResponse::ok(bus))
    }

    pub async fn delete(
        service: Data<RS>,
        path: Path<BusinessUnitPath>,
    ) -> ApiResult<()> {
        service
            .delete(&Actor::System, &path)
            .await
            .map(|_| ApiResponse::no_content())
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
        Ok(ApiResponse::created(bu))
    }
}
