use crate::{
    api::response::{CollectionResourceModel, Links, ResourceBuilder, SingleResourceModel},
    links::{Linkrelation, ResourceLink},
};
use actix_web::HttpRequest;
use derive_getters::Getters;
use perroute_commons::types::{code::Code, id::Id, vars::Vars};
use perroute_storage::models::channel::Channel;
use serde::{Deserialize, Serialize};
use std::ops::Deref;

#[derive(Debug, serde::Deserialize, Clone, Getters)]
pub struct CreateChannelRequest {
    code: Code,
    name: String,
    vars: Vars,
    enabled: bool,
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct UpdateChannelRequest {
    pub name: String,
    pub vars: Vars,
    pub enabled: bool,
}

#[derive(Clone, Serialize, Debug, Deserialize, PartialEq, Eq)]
pub struct ChannelResource {
    id: Id,
    code: Code,
    name: String,
    vars: Vars,
    enabled: bool,
}

impl From<Channel> for ChannelResource {
    fn from(value: Channel) -> Self {
        Self {
            id: value.id().to_owned(),
            code: value.code().clone(),
            name: value.name().clone(),
            vars: value.vars().deref().clone(),
            enabled: *value.enabled(),
        }
    }
}

impl ResourceBuilder<SingleResourceModel<ChannelResource>> for Channel {
    fn build(&self, req: &HttpRequest) -> SingleResourceModel<ChannelResource> {
        SingleResourceModel {
            data: Some(ChannelResource::from(self.clone())),
            links: Links::default()
                .add(Linkrelation::Self_, ResourceLink::Channel(*self.id()))
                .add(Linkrelation::Channels, ResourceLink::Channels)
                .as_url_map(req),
        }
    }
}

impl ResourceBuilder<CollectionResourceModel<ChannelResource>> for Vec<Channel> {
    fn build(&self, req: &HttpRequest) -> CollectionResourceModel<ChannelResource> {
        CollectionResourceModel {
            data: self.iter().map(|c| c.build(req)).collect(),
            links: Links::default()
                .add(Linkrelation::Self_, ResourceLink::Channels)
                .as_url_map(req),
        }
    }
}
