use crate::{
    api::response::{CollectionResourceModel, Links, ResourceBuilder, SingleResourceModel},
    links::{Linkrelation, ResourceLink},
};
use actix_web::HttpRequest;
use perroute_storage::models::business_unit::BusinessUnit;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use validator::Validate;

#[derive(Debug, serde::Deserialize, Clone, Validate)]
pub struct CreateBusinessUnitRequest {
    #[validate(custom = "perroute_commons::types::code::Code::validate")]
    pub code: String,
    pub name: String,
    pub vars: HashMap<String, String>,
}

#[derive(Debug, serde::Deserialize, Clone, Validate)]
pub struct UpdateBusinessUnitRequest {
    pub name: String,
    pub vars: HashMap<String, String>,
}

#[derive(Clone, Serialize, Debug, Deserialize, PartialEq, Eq)]
pub struct BusinessUnitResource {
    id: String,
    code: String,
    name: String,
    vars: HashMap<String, String>,
}

impl From<BusinessUnit> for BusinessUnitResource {
    fn from(value: BusinessUnit) -> Self {
        Self {
            id: value.id().into(),
            code: value.code().into(),
            name: value.name().clone(),
            vars: value.vars().into(),
        }
    }
}

impl ResourceBuilder<SingleResourceModel<BusinessUnitResource>> for BusinessUnit {
    fn build(&self, req: &HttpRequest) -> SingleResourceModel<BusinessUnitResource> {
        SingleResourceModel {
            data: Some(BusinessUnitResource::from(self.clone())),
            links: Links::default()
                .add(Linkrelation::Self_, ResourceLink::BusinessUnit(*self.id()))
                .add(Linkrelation::BusinessUnits, ResourceLink::BusinessUnits)
                .as_url_map(req),
        }
    }
}

impl ResourceBuilder<CollectionResourceModel<BusinessUnitResource>> for Vec<BusinessUnit> {
    fn build(&self, req: &HttpRequest) -> CollectionResourceModel<BusinessUnitResource> {
        CollectionResourceModel {
            data: self.iter().map(|c| c.build(req)).collect(),
            links: Links::default()
                .add(Linkrelation::Self_, ResourceLink::BusinessUnits)
                .as_url_map(req),
        }
    }
}
