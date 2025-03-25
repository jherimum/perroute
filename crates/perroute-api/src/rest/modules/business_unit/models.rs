use crate::rest::{
    error::ApiError,
    models::{
        link::{Relation, ResourcePath},
        resource::{ResourceModel, ResourceModelCollection},
    },
};
use chrono::NaiveDateTime;
use perroute_commons::types::{code::Code, id::Id, name::Name, vars::Vars};
use perroute_storage::models::business_unit::BusinessUnit;
use serde::{Deserialize, Serialize};
use url::Url;
use std::{collections::HashMap, fmt::Debug};
use validator::Validate;

use super::{BUSINESS_UNIT_COLLECTION_RESOURCE_NAME, BUSINESS_UNIT_RESOURCE_NAME};

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

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct BusinessUnitModel {
    pub id: String,
    pub code: String,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<BusinessUnit> for BusinessUnitModel {
    fn from(bu: BusinessUnit) -> Self {
        BusinessUnitModel {
            id: bu.id().to_string(),
            code: bu.code().to_string(),
            name: bu.name().to_string(),
            created_at: **bu.created_at(),
            updated_at: **bu.updated_at(),
        }
    }
}

impl From<BusinessUnit> for ResourceModel<BusinessUnitModel> {
    fn from(value: BusinessUnit) -> Self {
        ResourceModel::new(value.clone().into())
            .with_link(
                Relation::Self_,
                BusinessUnitPath::new(value.id().as_ref()),
            )
            .with_link(
                Relation::Static("business_units"),
                BusinessUnitCollectionPath,
            )
    }
}

impl From<Vec<BusinessUnit>> for ResourceModelCollection<BusinessUnitModel> {
    fn from(value: Vec<BusinessUnit>) -> Self {
        ResourceModelCollection::new(
            value.into_iter().map(Into::into).collect(),
        )
        .with_link(Relation::Self_, BusinessUnitCollectionPath)
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateBusinessUnitRequest {
    code: String,
    name: String,
    vars: HashMap<String, String>,
}

impl CreateBusinessUnitRequest {
    pub fn code(&self) -> Result<Code, ApiError> {
        Ok(Code::try_from(&self.code)?)
    }

    pub fn name(&self) -> Result<Name, ApiError> {
        Ok(Name::try_from(self.name.as_str())?)
    }

    pub fn vars(&self) -> Vars {
        Vars::from(&self.vars)
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateBusinessUnitRequest {
    name: String,
    vars: HashMap<String, String>,
}

impl UpdateBusinessUnitRequest {
    pub fn name(&self) -> Result<Name, ApiError> {
        Ok(Name::try_from(self.name.as_str())?)
    }

    pub fn vars(&self) -> Vars {
        Vars::from(&self.vars)
    }
}
