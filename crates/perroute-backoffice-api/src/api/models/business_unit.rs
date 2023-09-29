use crate::{
    api::response::{CollectionResourceModel, Links, ResourceBuilder, SingleResourceModel},
    links::{Linkrelation, ResourceLink},
};
use actix_web::HttpRequest;
use anyhow::{Context, Result};
use perroute_commons::types::{code::Code, vars::Vars};
use perroute_storage::models::business_unit::BusinessUnit;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, str::FromStr};
use validator::Validate;

use super::channel::ChannelRestQueryBuilder;

#[derive(Debug, serde::Deserialize, Clone, Validate, Default)]
#[serde(default)]
pub struct CreateBusinessUnitRequest {
    #[validate(required)]
    #[validate(custom = "Code::validate")]
    code: Option<String>,

    #[validate(required)]
    #[validate(custom = "perroute_commons::types::name::validate")]
    name: Option<String>,

    vars: Option<HashMap<String, String>>,
}
impl CreateBusinessUnitRequest {
    pub fn code(&self) -> Result<Code> {
        Code::from_str(self.code.as_ref().context("Missing code")?).context("Invalid code")
    }

    pub fn name(&self) -> Result<String> {
        self.name.clone().context("missing name")
    }

    pub fn vars(&self) -> Result<Vars> {
        Ok(self.vars.clone().unwrap_or_default().into())
    }
}

#[derive(Debug, serde::Deserialize, Clone, Validate)]
pub struct UpdateBusinessUnitRequest {
    #[validate(custom = "perroute_commons::types::name::validate")]
    name: Option<String>,
    vars: Option<HashMap<String, String>>,
}

impl UpdateBusinessUnitRequest {
    pub fn name(&self) -> Option<String> {
        self.name.clone()
    }

    pub fn vars(&self) -> Option<Vars> {
        self.vars.clone().map(|v| v.into())
    }
}

#[derive(Clone, Serialize, Debug, Deserialize, PartialEq, Eq)]
pub struct BusinessUnitResource {
    id: String,
    code: String,
    name: String,
    vars: HashMap<String, String>,
}

impl From<&BusinessUnit> for BusinessUnitResource {
    fn from(value: &BusinessUnit) -> Self {
        Self {
            id: value.id().into(),
            code: value.code().to_string(),
            name: value.name().clone(),
            vars: value.vars().into(),
        }
    }
}

impl ResourceBuilder<SingleResourceModel<BusinessUnitResource>> for BusinessUnit {
    fn build(&self, req: &HttpRequest) -> SingleResourceModel<BusinessUnitResource> {
        let channel_query = ChannelRestQueryBuilder::default()
            .business_unit_id(Some(self.id().to_string()))
            .build()
            .unwrap();

        SingleResourceModel {
            data: Some(self.into()),
            links: Links::default()
                .add(Linkrelation::Self_, ResourceLink::BusinessUnit(*self.id()))
                .add(Linkrelation::BusinessUnits, ResourceLink::BusinessUnits)
                .add(
                    Linkrelation::Channels,
                    ResourceLink::Channels(channel_query),
                )
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
