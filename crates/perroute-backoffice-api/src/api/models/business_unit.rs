use crate::{
    api::response::{CollectionResourceModel, Links, ResourceBuilder, SingleResourceModel},
    links::{Linkrelation, ResourceLink},
};
use actix_web::HttpRequest;
use derive_getters::Getters;
use perroute_commons::types::{code::Code, id::Id, vars::Vars};
use perroute_storage::models::business_unit::BusinessUnit;
use serde::{Deserialize, Serialize};
use std::ops::Deref;

#[derive(Debug, serde::Deserialize, Clone, Getters)]
pub struct CreateBusinessUnitRequest {
    code: Code,
    name: String,
    vars: Vars,
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct UpdateBusinessUnitRequest {
    pub name: String,
    pub vars: Vars,
}

#[derive(Clone, Serialize, Debug, Deserialize, PartialEq, Eq)]
pub struct BusinessUnitResource {
    id: Id,
    code: Code,
    name: String,
    vars: Vars,
}

impl From<BusinessUnit> for BusinessUnitResource {
    fn from(value: BusinessUnit) -> Self {
        Self {
            id: value.id().to_owned(),
            code: value.code().clone(),
            name: value.name().clone(),
            vars: value.vars().deref().clone(),
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
