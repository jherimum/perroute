use crate::api::{
    response::{CollectionResource, Resource, SingleResource},
    Linkrelation, ResourceLink,
};
use derive_getters::Getters;
use perroute_commons::types::code::Code;
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
    code: Code,
    description: String,
    enabled: bool,
}

impl Resource for MessageTypeResource {}

impl From<MessageType> for SingleResource<MessageTypeResource> {
    fn from(value: MessageType) -> Self {
        SingleResource::default()
            .with_data(value.clone().into())
            .with_link(
                Linkrelation::Self_,
                ResourceLink::MessageType(*value.channel_id(), *value.id()),
            )
            .with_link(
                Linkrelation::MessageTypes,
                ResourceLink::MessageTypes(*value.channel_id()),
            )
            .with_link(
                Linkrelation::Channel,
                ResourceLink::Channel(*value.channel_id()),
            )
            .with_link(
                Linkrelation::Schemas,
                ResourceLink::Schemas(*value.channel_id(), *value.id()),
            )
    }
}

impl From<(Channel, Vec<MessageType>)> for CollectionResource<MessageTypeResource> {
    fn from(value: (Channel, Vec<MessageType>)) -> Self {
        CollectionResource::default()
            .with_link(
                Linkrelation::Self_,
                ResourceLink::MessageTypes(*value.0.id()),
            )
            .with_link(Linkrelation::Channel, ResourceLink::Channel(*value.0.id()))
            .with_resources(value.1.into_iter().map(MessageType::into).collect())
    }
}

impl From<MessageType> for MessageTypeResource {
    fn from(value: MessageType) -> Self {
        MessageTypeResource {
            code: value.code().to_owned(),
            description: value.description().to_owned(),
            enabled: *value.enabled(),
        }
    }
}
