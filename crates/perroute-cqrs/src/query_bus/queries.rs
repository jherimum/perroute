use perroute_commons::types::{code::Code, id::Id};
use perroute_storage::models::schema::Version;
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

query!(
    FindMessageTypeQuery,
    QueryType::FindMessageTypeQuery,
    message_type_id: Id,
    channel_id: Option<Id>
);

query!(
    QueryMessageTypesQuery,
    QueryType::QueryMessageTypes,
    channel_id: Option<Id>
);

query!(
    QuerySchemasQuery,
    QueryType::QuerySchemas,
    message_type_id: Id
);

query!(
    FindSchemaQuery,
    QueryType::FindSchema,
    channel_id: Option<Id>,
    message_type_id: Option<Id>,
    message_type_code: Option<Code>,
    version: Option<Version>,
    schema_id: Option<Id>,
    channel_code: Option<Code>
);

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

#[derive(
    Debug, serde::Serialize, Clone, PartialEq, Eq, derive_builder::Builder, derive_getters::Getters,
)]
pub struct FindApiKeyQuery {
    #[builder(default)]
    api_key_id: Option<Id>,
    #[builder(default)]
    key: Option<String>,
}

impl_query!(FindApiKeyQuery, QueryType::FindApiKey);

#[derive(
    Debug, serde::Serialize, Clone, PartialEq, Eq, derive_builder::Builder, derive_getters::Getters,
)]
pub struct QueryApiKeysQuery {
    #[builder(default)]
    api_key_id: Option<Id>,

    #[builder(default)]
    channel_id: Option<Id>,

    #[builder(default)]
    key: Option<String>,
}

impl_query!(QueryApiKeysQuery, QueryType::QueryApiKeys);
