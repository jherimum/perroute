use crate::{
    api::response::{CollectionResourceModel, Links, ResourceBuilder, SingleResourceModel},
    links::{Linkrelation, ResourceLink},
};
use perroute_storage::models::message_type::MessageType;
use serde::Serialize;
use std::collections::HashMap;
use validator::Validate;

#[derive(Debug, serde::Deserialize, Clone, Validate, Default)]
#[serde(default)]
pub struct CreateMessageTypeRequest {
    #[validate(required)]
    #[validate(custom = "perroute_commons::types::code::Code::validate")]
    pub code: Option<String>,

    #[validate(required)]
    #[validate(custom = "perroute_commons::types::name::validate")]
    pub name: Option<String>,

    pub vars: Option<HashMap<String, String>>,

    #[validate(required)]
    #[validate(custom = "perroute_commons::types::id::Id::validate")]
    pub business_unit_id: Option<String>,
}

#[derive(Debug, serde::Deserialize, Clone, Validate, Default)]
#[serde(default)]
pub struct UpdateMessageTypeRequest {
    #[validate(custom = "perroute_commons::types::name::validate")]
    pub name: Option<String>,

    pub enabled: Option<bool>,

    pub vars: Option<HashMap<String, String>>,
}

#[derive(Clone, Serialize, Debug, Validate)]
pub struct MessageTypeResource {
    id: String,
    code: String,
    name: String,
    enabled: bool,
    vars: HashMap<String, String>,
}

impl From<MessageType> for MessageTypeResource {
    fn from(value: MessageType) -> Self {
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
            data: Some(self.clone().into()),
            links: Links::default()
                .add(Linkrelation::Self_, ResourceLink::MessageType(*self.id()))
                .add(Linkrelation::MessageTypes, ResourceLink::MessageTypes)
                .add(Linkrelation::Schemas, ResourceLink::Schemas(*self.id()))
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
