use std::fmt::Debug;

use derive_builder::Builder;
use derive_getters::Getters;
use perroute_commons::types::id::Id;
use serde::Serialize;
use strum_macros::Display;

macro_rules! impl_query {
    ($cmd: ty, $ty: expr) => {
        impl Query for $cmd {
            fn ty(&self) -> QueryType {
                $ty
            }
        }
    };
}

pub trait Query: Debug + Serialize + Clone + PartialEq + Eq + Send + Sync {
    fn ty(&self) -> QueryType;
}

#[derive(Debug, Clone, PartialEq, Eq, Display)]
pub enum QueryType {
    FindChannel,
    QueryChannels,
    FindMessageTypeQuery,
    QueryChannelMessageTypes,
    QuerySchemas,
    FindSchema,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Builder, Getters)]
pub struct FindChannelQuery {
    channel_id: Id,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Builder)]
pub struct QueryChannelsQuery {}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Builder, Getters)]
pub struct FindMessageTypeQuery {
    message_type_id: Id,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Builder, Getters)]
pub struct QueryMessageTypesQuery {
    channel_id: Id,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Builder, Getters)]
pub struct QuerySchemasQuery {
    message_type_id: Id,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Builder, Getters)]
pub struct FindSchemaQuery {
    message_type_id: Id,
    version: i32,
}

impl_query!(FindChannelQuery, QueryType::FindChannel);
impl_query!(QueryChannelsQuery, QueryType::QueryChannels);
impl_query!(FindMessageTypeQuery, QueryType::FindMessageTypeQuery);
impl_query!(QueryMessageTypesQuery, QueryType::QueryChannelMessageTypes);
impl_query!(QuerySchemasQuery, QueryType::QuerySchemas);
impl_query!(FindSchemaQuery, QueryType::FindSchema);
