use perroute_commons::types::{code::Code, id::Id};
use serde::Serialize;
use std::fmt::Debug;
use strum_macros::Display;

use crate::query;

pub trait Query: Debug + Serialize + Clone + PartialEq + Eq + Send + Sync {
    fn ty(&self) -> QueryType;
}

#[derive(Debug, Clone, PartialEq, Eq, Display)]
pub enum QueryType {
    FindChannelByCode,
    FindChannelById,
    QueryChannels,

    FindMessageTypeQuery,
    QueryMessageTypes,

    QuerySchemas,
    FindSchema,
    FindSchemaById,
    FindChannelMessageTypeSchema,

    QueryTemplates,
    FindTemplate,
}

query!(
    FindChannelByCodeQuery,
    QueryType::FindChannelByCode,
    channel_code: Code
);
query!(
    FindChannelByIdQuery,
    QueryType::FindChannelById,
    channel_id: Id
);

query!(QueryChannelsQuery, QueryType::QueryChannels);

query!(
    FindMessageTypeQuery,
    QueryType::FindMessageTypeQuery,
    message_type_id: Id
);

query!(QueryMessageTypesQuery, QueryType::QueryMessageTypes);

query!(
    QueryMessageTypeSchemasQuery,
    QueryType::QuerySchemas,
    message_type_id: Id
);

query!(
    FindSchemaQuery,
    QueryType::FindSchema,
    channel_id: Id,
    message_type_id: Id,
    version: Id
);

query!(
    FindMessageTypeSchemaQuery,
    QueryType::FindChannelMessageTypeSchema,
    message_type_id: Id,
    schema_id: Id
);

query!(QueryTemplatesQuery, QueryType::QueryTemplates);

query!(FindTemplateQuery, QueryType::FindTemplate, template_id: Id);
