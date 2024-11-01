use std::collections::HashMap;
use bon::Builder;
use chrono::NaiveDateTime;
use perroute_storage::models::channel::Channel;
use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::rest::models::ResourceModel;


#[derive(Debug, Deserialize, Validate)]
pub struct ChannelPath{
    business_unit_id: String,
    channel_id: String,
}

impl ChannelPath{
    pub fn parent(&self) -> ChannelCollectionPath{
        ChannelCollectionPath{
            business_unit_id: self.business_unit_id.clone(),
        }
    }
}


#[derive(Debug, Deserialize, Validate)]
pub struct ChannelCollectionPath{
    business_unit_id: String,
}



#[derive(Debug, Serialize, Builder)]
pub struct ChannelModel{
    pub id: String,
    pub business_unit_id: String,
    pub name: String,
    pub provider_id: String,
    pub dispatch_type: String,
    pub configuration: HashMap<String, String>,
    pub enabled: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<&Channel> for ChannelModel{
    fn from(value: &Channel) -> Self {
        ChannelModel::builder()
        .id(value.id.to_string())
        .business_unit_id(value.business_unit_id.to_string())
        .name(value.name.to_string())
        .provider_id(value.provider_id.to_string())
        .dispatch_type(value.dispatch_type.to_string())
        .configuration((*value.configuration).clone())
        .enabled(value.enabled)
        .created_at(*value.created_at)
        .updated_at(*value.updated_at)        
        .build()
    }
}

impl From<&Channel> for ResourceModel<ChannelModel>{
    fn from(value: &Channel) -> Self {
        ResourceModel::new(value.into())
    }
}



#[derive(Debug, Deserialize, Validate)]
pub struct CreateChannelRequest{
    pub name: String,
    pub provider_id: String,
    pub dispatch_type: String,
    pub configuration: HashMap<String, String>,
    pub enabled: bool,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateChannelRequest{
    pub name: String,
    pub configuration: HashMap<String, String>,
    pub enabled: bool,
}