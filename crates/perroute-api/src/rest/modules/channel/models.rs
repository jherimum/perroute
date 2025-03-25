use crate::rest::{
    error::ApiError, models::resource::ResourceModel,
    modules::business_unit::models::BusinessUnitPath,
};
use bon::Builder;
use chrono::NaiveDateTime;
use perroute_commons::types::{
    dispatch_type::DispatchType, id::Id, name::Name, Configuration, ProviderId,
};
use perroute_storage::models::channel::Channel;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, str::FromStr};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct ChannelPath {
    business_unit_id: String,
    channel_id: String,
}

impl ChannelPath {
    pub fn business_unit_id(&self) -> Id {
        Id::from(&self.business_unit_id)
    }

    pub fn channel_id(&self) -> Id {
        Id::from(&self.channel_id)
    }
}

impl ChannelPath {
    pub fn channel_collection_path(&self) -> ChannelCollectionPath {
        ChannelCollectionPath {
            business_unit_id: self.business_unit_id.to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct ChannelCollectionPath {
    business_unit_id: String,
}

impl ChannelCollectionPath {
    pub fn new(business_unit_id: &str) -> Self {
        ChannelCollectionPath {
            business_unit_id: business_unit_id.to_string(),
        }
    }

    pub fn business_unit_id(&self) -> Id {
        Id::from(&self.business_unit_id)
    }

    pub fn business_unit_path(&self) -> BusinessUnitPath {
        BusinessUnitPath::new(&self.business_unit_id)
    }
}

#[derive(Debug, Serialize, Builder)]
pub struct ChannelModel {
    id: String,
    business_unit_id: String,
    name: String,
    provider_id: String,
    dispatch_type: String,
    configuration: HashMap<String, String>,
    enabled: bool,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

impl From<Channel> for ChannelModel {
    fn from(value: Channel) -> Self {
        ChannelModel::builder()
            .id(value.id().to_string())
            .business_unit_id(value.business_unit_id().to_string())
            .name(value.name().to_string())
            .provider_id(value.provider_id().to_string())
            .dispatch_type(value.dispatch_type().to_string())
            .configuration((***value.configuration()).clone())
            .enabled(*value.enabled())
            .created_at(**value.created_at())
            .updated_at(**value.updated_at())
            .build()
    }
}

impl From<Channel> for ResourceModel<ChannelModel> {
    fn from(value: Channel) -> Self {
        ResourceModel::new(value.into())
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateChannelRequest {
    name: String,
    provider_id: String,
    dispatch_type: String,
    configuration: HashMap<String, String>,
    enabled: bool,
}

impl CreateChannelRequest {
    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn name(&self) -> Result<Name, ApiError> {
        Ok(Name::try_from(self.name.as_str())?)
    }

    pub fn provider_id(&self) -> ProviderId {
        ProviderId::new(&self.provider_id)
    }

    pub fn dispatch_type(&self) -> Result<DispatchType, ApiError> {
        Ok(DispatchType::from_str(&self.dispatch_type)?)
    }

    pub fn configuration(&self) -> Configuration {
        Configuration::new(&self.configuration)
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateChannelRequest {
    name: String,
    configuration: HashMap<String, String>,
    enabled: bool,
}

impl UpdateChannelRequest {
    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn name(&self) -> Result<Name, ApiError> {
        Ok(Name::try_from(self.name.as_str())?)
    }

    pub fn configuration(&self) -> Configuration {
        Configuration::new(&self.configuration)
    }
}
