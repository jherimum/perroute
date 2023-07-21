use derive_builder::Builder;
use derive_getters::Getters;
use perroute_commons::types::{code::Code, id::Id};
use perroute_storage::models::schema::Version;
use serde::Serialize;
use std::fmt::Debug;
use strum_macros::Display;

use crate::{impl_query, query};

pub trait Query {
    fn ty(&self) -> QueryType;
}

#[derive(Debug, Clone, PartialEq, Eq, Display)]
pub enum QueryType {
    FindChannel,
    QueryChannels,

    FindMessageTypeQuery,
    QueryMessageTypes,

    QuerySchemas,
    FindSchema,

    QueryTemplates,
    FindTemplate,

    QueryApiKeys,
    FindApiKey,
}

query!(
    FindChannelQuery,
    QueryType::FindChannel,
    channel_id: Option<Id>
);
query!(QueryChannelsQuery, QueryType::QueryChannels);

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Builder, Getters)]
pub struct FindMessageTypeQuery {
    message_type_id: Id,
    #[builder(default)]
    channel_id: Option<Id>,
}

impl_query!(FindMessageTypeQuery, QueryType::FindMessageTypeQuery);

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Builder, Getters)]
pub struct QueryMessageTypesQuery {
    #[builder(default)]
    channel_id: Option<Id>,
}

impl_query!(QueryMessageTypesQuery, QueryType::QueryMessageTypes);

query!(
    QuerySchemasQuery,
    QueryType::QuerySchemas,
    message_type_id: Id
);

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Builder, Getters)]
pub struct FindSchemaQuery {
    #[builder(default)]
    channel_id: Option<Id>,
    #[builder(default)]
    message_type_id: Option<Id>,
    #[builder(default)]
    message_type_code: Option<Code>,
    #[builder(default)]
    version: Option<Version>,
    #[builder(default)]
    schema_id: Option<Id>,
    #[builder(default)]
    channel_code: Option<Code>,
}

impl_query!(FindSchemaQuery, QueryType::FindSchema);

query!(
    QueryTemplatesQuery,
    QueryType::QueryTemplates,
    schema_id: Option<Id>
);

query!(
    FindTemplateQuery,
    QueryType::FindTemplate,
    template_id: Id,
    schema_id: Option<Id>,
    message_type_id: Option<Id>,
    channel_id: Option<Id>
);

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Builder, Getters)]
pub struct FindApiKeyQuery {
    #[builder(default)]
    api_key_id: Option<Id>,
    #[builder(default)]
    key: Option<String>,
}

impl_query!(FindApiKeyQuery, QueryType::FindApiKey);

#[derive(Debug, serde::Serialize, Clone, PartialEq, Eq, Builder, derive_getters::Getters)]
pub struct QueryApiKeysQuery {
    #[builder(default)]
    api_key_id: Option<Id>,
    #[builder(default)]
    channel_id: Option<Id>,
    #[builder(default)]
    key: Option<String>,
}

impl_query!(QueryApiKeysQuery, QueryType::QueryApiKeys);
