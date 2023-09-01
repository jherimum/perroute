use crate::{
    api::response::{CollectionResourceModel, Links, ResourceBuilder, SingleResourceModel},
    links::{Linkrelation, ResourceLink},
};
use anyhow::{Context, Result};
use perroute_commons::types::{code::Code, id::Id, vars::Vars};
use perroute_storage::models::message_type::MessageType;
use serde::Serialize;
use std::{collections::HashMap, str::FromStr};
use validator::Validate;

#[derive(Debug, serde::Deserialize, Clone, Validate, Default)]
#[serde(default)]
pub struct CreateMessageTypeRequest {
    #[validate(required)]
    #[validate(custom = "Code::validate")]
    code: Option<String>,

    #[validate(required)]
    #[validate(custom = "perroute_commons::types::name::validate")]
    name: Option<String>,

    vars: Option<HashMap<String, String>>,

    #[validate(required)]
    #[validate(custom = "Id::validate")]
    business_unit_id: Option<String>,
}

impl CreateMessageTypeRequest {
    pub fn code(&self) -> Result<Code> {
        Code::from_str(self.code.as_ref().context("Missing code")?).context("Invalid code")
    }

    pub fn name(&self) -> Result<String> {
        self.name.clone().context("Missing name")
    }

    pub fn business_unit_id(&self) -> Result<Id> {
        self.business_unit_id
            .clone()
            .context("Missing business id")?
            .try_into()
            .context("Invalid Id")
    }

    pub fn vars(&self) -> Result<Vars> {
        Ok(self.vars.clone().unwrap_or_default().into())
    }
}
#[derive(Debug, serde::Deserialize, Clone, Validate, Default)]
#[serde(default)]
pub struct UpdateMessageTypeRequest {
    #[validate(custom = "perroute_commons::types::name::validate")]
    name: Option<String>,

    enabled: Option<bool>,

    vars: Option<HashMap<String, String>>,
}

impl UpdateMessageTypeRequest {
    pub fn vars(&self) -> Result<Option<Vars>> {
        Ok(self.vars.clone().map(Into::into))
    }

    pub fn name(&self) -> Result<Option<String>> {
        Ok(self.name.clone())
    }

    pub fn enabled(&self) -> Result<Option<bool>> {
        Ok(self.enabled)
    }
}

#[derive(Clone, Serialize, Debug, Validate)]
pub struct MessageTypeResource {
    id: String,
    code: String,
    name: String,
    enabled: bool,
    vars: HashMap<String, String>,
}

impl From<&MessageType> for MessageTypeResource {
    fn from(value: &MessageType) -> Self {
        Self {
            id: value.id().into(),
            code: value.code().to_string(),
            name: value.name().clone(),
            enabled: *value.enabled(),
            vars: value.vars().into(),
        }
    }
}

impl ResourceBuilder<SingleResourceModel<MessageTypeResource>> for MessageType {
    fn build(&self, req: &actix_web::HttpRequest) -> SingleResourceModel<MessageTypeResource> {
        SingleResourceModel {
            data: Some(self.into()),
            links: Links::default()
                .add(Linkrelation::Self_, ResourceLink::MessageType(*self.id()))
                .add(Linkrelation::MessageTypes, ResourceLink::MessageTypes)
                .add(
                    Linkrelation::BusinessUnit,
                    ResourceLink::BusinessUnit(*self.business_unit_id()),
                )
                .as_url_map(req),
        }
    }
}

impl ResourceBuilder<CollectionResourceModel<MessageTypeResource>> for Vec<MessageType> {
    fn build(&self, req: &actix_web::HttpRequest) -> CollectionResourceModel<MessageTypeResource> {
        CollectionResourceModel {
            data: self.iter().map(|c| c.build(req)).collect(),
            links: Links::default()
                .add(Linkrelation::Self_, ResourceLink::MessageTypes)
                .as_url_map(req),
        }
    }
}
