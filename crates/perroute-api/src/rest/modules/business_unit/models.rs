use crate::rest::{
    error::ApiError,
    models::{
        link::{Relation, ResourcePath},
        resource::ResourceModel,
    },
};
use chrono::NaiveDateTime;
use perroute_commons::types::{code::Code, name::Name, vars::Vars};
use perroute_storage::models::business_unit::BusinessUnit;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Debug};
use validator::Validate;

use super::controller::{BusinessUnitCollectionPath, BusinessUnitPath};

#[derive(Debug, Serialize, Deserialize)]
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
                BusinessUnitPath::new(value.id().to_string().as_str()),
            )
            .with_link(
                Relation::Static("business_units"),
                BusinessUnitCollectionPath,
            )
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateBusinessUnitRequest {
    code: String,
    name: String,
    vars: Option<HashMap<String, String>>,
}

impl CreateBusinessUnitRequest {
    pub fn code(&self) -> Result<Code, ApiError> {
        Ok(Code::try_from(&self.code)?)
    }

    pub fn name(&self) -> Result<Name, ApiError> {
        Ok(Name::try_from(&self.name)?)
    }

    pub fn vars(&self) -> Result<Option<Vars>, ApiError> {
        Ok(self.vars.as_ref().map(From::from))
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateBusinessUnitRequest {
    name: String,
    vars: Option<HashMap<String, String>>,
}

impl UpdateBusinessUnitRequest {
    pub fn name(&self) -> Result<Name, ApiError> {
        Ok(Name::try_from(&self.name)?)
    }

    pub fn vars(&self) -> Result<Option<Vars>, ApiError> {
        Ok(self.vars.as_ref().map(From::from))
    }
}
