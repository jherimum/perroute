use std::fmt::Debug;
use strum_macros::Display;

pub trait Query {
    fn ty(&self) -> QueryType;
}

#[derive(Debug, Clone, PartialEq, Eq, Display)]
pub enum QueryType {
    FindBusinessUnit,
    QueryBusinessUnits,

    FindMessageTypeQuery,
    QueryMessageTypes,

    QuerySchemas,
    FindSchema,

    QueryTemplates,
    FindTemplate,

    QueryApiKeys,
    FindApiKey,
}
