use std::ops::Deref;

use crate::{
    api::response::{CollectionResourceModel, Links, ResourceBuilder, SingleResourceModel},
    links::{Linkrelation, ResourceLink},
};
use derive_getters::Getters;
use perroute_commons::types::{code::Code, id::Id, vars::Vars};
use perroute_storage::models::message_type::MessageType;
use serde::Serialize;

#[derive(Debug, serde::Deserialize, Clone, Getters)]
pub struct CreateMessageTypeRequest {
    code: Code,
    name: String,
    vars: Vars,
    bu_id: Id,
}

#[derive(Debug, serde::Deserialize, Clone, Getters)]
pub struct UpdateMessageTypeRequest {
    name: String,
    enabled: bool,
    vars: Vars,
}

#[derive(Clone, Serialize, Debug)]
pub struct MessageTypeResource {
    id: Id,
    code: Code,
    name: String,
    enabled: bool,
    vars: Vars,
}

impl From<MessageType> for MessageTypeResource {
    fn from(value: MessageType) -> Self {
        Self {
            id: *value.id(),
            code: value.code().clone(),
            name: value.name().clone(),
            enabled: *value.enabled(),
            vars: value.vars().deref().clone(),
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
