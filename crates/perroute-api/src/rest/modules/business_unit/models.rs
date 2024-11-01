use chrono::NaiveDateTime;
use perroute_commons::types::{id::Id, name::Name, vars::Vars, Code};
use perroute_storage::models::business_unit::BusinessUnit;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Debug};
use validator::Validate;

use crate::rest::{error::ApiError, models::ResourceModel};

#[derive(Debug, Deserialize)]
pub struct BusinessUnitPath(String);

impl BusinessUnitPath {
    pub fn id(&self) -> Id {
        Id::from(self.0.clone())
    }
}

#[derive(Debug, Deserialize)]
pub struct BusinessUnitCollectionPath;

#[derive(Debug, Serialize, Deserialize)]
pub struct BusinessUnitModel {
    pub id: String,
    pub code: String,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<&BusinessUnit> for BusinessUnitModel {
    fn from(bu: &BusinessUnit) -> Self {
        BusinessUnitModel {
            id: bu.id.to_string(),
            code: bu.code.to_string(),
            name: bu.name.to_string(),
            created_at: *bu.created_at,
            updated_at: *bu.updated_at,
        }
    }
}

impl From<&BusinessUnit> for ResourceModel<BusinessUnitModel> {
    fn from(value: &BusinessUnit) -> Self {
        ResourceModel::new(value.into())
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateBusinessUnitRequest {
    pub code: String,
    pub name: String,
    pub vars: Option<HashMap<String, String>>,
}

impl CreateBusinessUnitRequest {
    pub fn code(&self) -> Result<Code, ApiError> {
        Code::try_from(self.code.clone()).map_err(|_| ApiError::BadRequest)
    }

    // pub fn name(&self) -> Result<Name, ApiError> {
    //     Name::try_from(self.name.clone()).map_err(|_| ApiError::BadRequest)
    // }

    pub fn vars(&self) -> Option<Vars> {
        self.vars.as_ref().map(|v| Vars::new(v.clone()))
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateBusinessUnitRequest {
    pub name: String,
    pub vars: Option<HashMap<String, String>>,
}

impl UpdateBusinessUnitRequest {
    pub fn vars(&self) -> Option<Vars> {
        self.vars.as_ref().map(|v| Vars::new(v.clone()))
    }
}
