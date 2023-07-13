use perroute_commons::types::id::Id;
use serde::Serialize;
use std::fmt::Debug;
use strum_macros::Display;

use crate::query;

pub trait Query: Debug + Serialize + Clone + PartialEq + Eq + Send + Sync {
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
    schema_id: Id
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

query!(FindApiKeyQuery, QueryType::FindApiKey, api_key_id: Id);
query!(QueryApiKeysQuery, QueryType::QueryApiKeys, api_key_id: Id);
