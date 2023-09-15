use crate::{
    api::response::{CollectionResourceModel, Links, ResourceBuilder, SingleResourceModel},
    links::{Linkrelation, ResourceLink},
};
use actix_web::HttpRequest;
use anyhow::{Context, Result};
use derive_builder::Builder;
use perroute_commons::types::{id::Id, priority::Priority, properties::Properties};
use perroute_connectors::types::dispatch_type::DispatchType;
use perroute_storage::models::channel::Channel;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use validator::Validate;

#[derive(Debug, Default, Deserialize, Validate, Clone, Builder, Serialize)]
pub struct ChannelRestQuery {
    #[validate(custom = "Id::validate")]
    pub business_unit_id: Option<String>,
}

impl ChannelRestQuery {
    pub fn business_unit_id(&self) -> Result<Option<Id>> {
        Ok(self
            .business_unit_id
            .clone()
            .map(|id| id.try_into())
            .transpose()?)
    }
}

#[derive(Debug, Deserialize, Clone, Validate, Default)]
#[serde(default)]
pub struct CreateChannelRequest {
    #[validate(required)]
    #[validate(custom = "Id::validate")]
    business_id: Option<String>,

    #[validate(required)]
    #[validate(custom = "Id::validate")]
    connection_id: Option<String>,

    #[validate(required)]
    #[validate(custom = "DispatchType::validate")]
    dispatch_type: Option<String>,

    #[validate(required)]
    #[validate(custom = "Properties::validate")]
    properties: Option<Value>,

    #[validate(required)]
    #[validate(custom = "Priority::validate")]
    priority: Option<i32>,
}

impl CreateChannelRequest {
    pub fn into_business_id(&self) -> Result<Id> {
        self.business_id
            .clone()
            .context("Missing business id")?
            .try_into()
            .context("Invalid Id")
    }

    pub fn into_connection_id(&self) -> Result<Id> {
        self.connection_id
            .clone()
            .context("Missing connection id")?
            .try_into()
            .context("Invalid Id")
    }

    pub fn into_dispatch_type(&self) -> Result<DispatchType> {
        self.dispatch_type
            .clone()
            .context("Missing dispatch type")?
            .try_into()
            .context("Invalid dispatch type")
    }

    pub fn into_properties(&self) -> Result<Properties> {
        self.properties
            .clone()
            .context("Missing properties")?
            .try_into()
            .context("Invalid properties")
    }

    pub fn into_priority(&self) -> Result<Priority> {
        self.priority
            .context("Missing priority")?
            .try_into()
            .context("Invalid priority")
    }
}

#[derive(Debug, Deserialize, Clone, Validate, Default)]
#[serde(default)]
pub struct UpdateChannelRequest {
    #[validate(required)]
    #[validate(custom = "perroute_commons::types::properties::Properties::validate")]
    properties: Option<Value>,

    #[validate(custom = "perroute_commons::types::priority::Priority::validate")]
    priority: Option<i32>,

    enabled: Option<bool>,
}

impl UpdateChannelRequest {
    pub fn into_properties(&self) -> Result<Option<Properties>> {
        Ok(self.properties.clone().map(|p| p.try_into()).transpose()?)
    }

    pub fn into_priority(&self) -> Result<Option<Priority>> {
        Ok(self.priority.map(|p| p.try_into()).transpose()?)
    }

    pub fn into_enabled(&self) -> Option<bool> {
        self.enabled
    }
}

#[derive(Clone, Serialize, Debug, Deserialize, PartialEq, Eq)]
pub struct ChannelResource {
    id: String,
    dispatch_type: String,
    properties: Value,
    enabled: bool,
    priority: i32,
}

impl From<&Channel> for ChannelResource {
    fn from(value: &Channel) -> Self {
        Self {
            id: value.id().into(),
            dispatch_type: value.dispatch_type().into(),
            properties: value.properties().into(),
            enabled: *value.enabled(),
            priority: value.priority().into(),
        }
    }
}

impl ResourceBuilder<SingleResourceModel<ChannelResource>> for Channel {
    fn build(&self, req: &HttpRequest) -> SingleResourceModel<ChannelResource> {
        SingleResourceModel {
            data: Some(self.into()),
            links: Links::default()
                .add(Linkrelation::Self_, ResourceLink::Channel(*self.id()))
                .add(
                    Linkrelation::Channels,
                    ResourceLink::Channels(Default::default()),
                )
                .add(
                    Linkrelation::Connection,
                    ResourceLink::Connection(*self.connection_id()),
                )
                .add(
                    Linkrelation::BusinessUnit,
                    ResourceLink::BusinessUnit(*self.business_unit_id()),
                )
                .as_url_map(req),
        }
    }
}

impl ResourceBuilder<CollectionResourceModel<ChannelResource>>
    for (Vec<Channel>, ChannelRestQuery)
{
    fn build(&self, req: &HttpRequest) -> CollectionResourceModel<ChannelResource> {
        CollectionResourceModel {
            data: self.0.iter().map(|c| c.build(req)).collect(),
            links: Links::default()
                .add(Linkrelation::Self_, ResourceLink::Channels(self.1.clone()))
                .as_url_map(req),
        }
    }
}
