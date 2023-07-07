use crate::api::{
    response::{CollectionResourceModel, Links, ResourceBuilder, SingleResourceModel},
    Linkrelation, ResourceLink,
};
use derive_getters::Getters;
use perroute_commons::{prelude::Id, types::code::Code};
use perroute_storage::models::{channel::Channel, message_type::MessageType};
use serde::Serialize;

#[derive(Debug, serde::Deserialize, Clone, Getters)]
pub struct CreateMessageTypeRequest {
    code: Code,
    description: String,
}

#[derive(Debug, serde::Deserialize, Clone, Getters)]
pub struct UpdateMessageTypeRequest {
    description: String,
    enabled: bool,
}

#[derive(Clone, Serialize, Debug)]
pub struct MessageTypeResource {
    id: Id,
    code: Code,
    description: String,
    enabled: bool,
}

impl From<MessageType> for MessageTypeResource {
    fn from(value: MessageType) -> Self {
        MessageTypeResource {
            id: value.id().to_owned(),
            code: value.code().to_owned(),
            description: value.description().to_owned(),
            enabled: *value.enabled(),
        }
    }
}

impl ResourceBuilder<SingleResourceModel<MessageTypeResource>> for MessageType {
    fn build(&self, req: &actix_web::HttpRequest) -> SingleResourceModel<MessageTypeResource> {
        SingleResourceModel {
            data: Some(self.clone().into()),
            links: Links::default()
                .add(
                    Linkrelation::Self_,
                    ResourceLink::MessageType(*self.channel_id(), *self.id()),
                )
                .add(
                    Linkrelation::MessageTypes,
                    ResourceLink::MessageTypes(*self.channel_id()),
                )
                .add(
                    Linkrelation::Schemas,
                    ResourceLink::Schemas(*self.channel_id(), *self.id()),
                )
                .add(
                    Linkrelation::Channel,
                    ResourceLink::Channel(*self.channel_id()),
                )
                .as_url_map(req),
        }
    }
}

impl ResourceBuilder<CollectionResourceModel<MessageTypeResource>> for (Channel, Vec<MessageType>) {
    fn build(&self, req: &actix_web::HttpRequest) -> CollectionResourceModel<MessageTypeResource> {
        CollectionResourceModel {
            data: self.1.iter().map(|c| c.build(req)).collect(),
            links: Links::default()
                .add(
                    Linkrelation::Self_,
                    ResourceLink::MessageTypes(*self.0.id()),
                )
                .add(Linkrelation::Channel, ResourceLink::Channel(*self.0.id()))
                .as_url_map(req),
        }
    }
}
