use crate::errors::PerrouteBackofficeApiError;
use axum::{
    extract::{FromRequestParts, Path},
    http::request::Parts,
};
use perroute_commons::{
    rest::RestError,
    types::{actor::Actor, id::Id},
};
use perroute_cqrs::query_bus::{
    bus::{QueryBus, QueryHandler},
    handlers::{
        channel::find_channel::FindChannelQueryHandler,
        message_type::find_message_type::FindMessageTypeQueryHandler,
        schema::find_schema::FindSchemaQueryHandler,
    },
    queries::{
        FindChannelQuery, FindChannelQueryBuilder, FindMessageTypeQuery,
        FindMessageTypeQueryBuilder, FindSchemaQuery, FindSchemaQueryBuilder, Query,
    },
};
use perroute_storage::models::{channel::Channel, message_type::MessageType, schema::Schema};
use std::fmt::Debug;

#[async_trait::async_trait]
pub trait ResourcePath {
    type Resource: Debug;
    type Handler: QueryHandler<Query = Self::Query, Output = Option<Self::Resource>>
        + 'static
        + Sync
        + Send;
    type Query: Query + 'static + Sync + Send;

    async fn fetch(
        &self,
        query_bus: &QueryBus,
        actor: &Actor,
        when_none: impl FnOnce() -> RestError + Send + Sync,
    ) -> Result<Self::Resource, RestError> {
        query_bus
            .execute::<_, Self::Handler, _>(actor, self.query())
            .await
            .map_err(PerrouteBackofficeApiError::from)?
            .ok_or_else(when_none)
    }

    fn query(&self) -> Self::Query;
}

#[async_trait::async_trait]
impl<S> FromRequestParts<S> for ChannelPath
where
    S: Send + Sync,
{
    type Rejection = RestError;
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let path = <Path<Id>>::from_request_parts(parts, state).await.unwrap();
        Ok(path.into())
    }
}

#[derive(Debug)]
pub struct ChannelPath {
    channel_id: Id,
}

impl From<Path<Id>> for ChannelPath {
    fn from(value: Path<Id>) -> Self {
        ChannelPath::from(value.0)
    }
}

impl From<Id> for ChannelPath {
    fn from(channel_id: Id) -> Self {
        ChannelPath { channel_id }
    }
}

#[async_trait::async_trait]
impl ResourcePath for ChannelPath {
    type Resource = Channel;
    type Handler = FindChannelQueryHandler;
    type Query = FindChannelQuery;

    fn query(&self) -> Self::Query {
        FindChannelQueryBuilder::default()
            .channel_id(self.channel_id)
            .build()
            .unwrap()
    }
}

#[derive(Debug)]
pub struct MessageTypePath {
    pub channel_id: Id,
    pub message_type_id: Id,
}

impl ResourcePath for MessageTypePath {
    type Resource = MessageType;

    type Handler = FindMessageTypeQueryHandler;

    type Query = FindMessageTypeQuery;

    fn query(&self) -> Self::Query {
        FindMessageTypeQueryBuilder::default()
            .message_type_id(self.message_type_id)
            .channel_id(Some(self.channel_id))
            .build()
            .unwrap()
    }
}

impl From<Path<(Id, Id)>> for MessageTypePath {
    fn from(value: Path<(Id, Id)>) -> Self {
        MessageTypePath::from(value.0)
    }
}

impl From<(Id, Id)> for MessageTypePath {
    fn from((channel_id, message_type_id): (Id, Id)) -> Self {
        MessageTypePath {
            channel_id,
            message_type_id,
        }
    }
}

#[async_trait::async_trait]
impl<S> FromRequestParts<S> for MessageTypePath
where
    S: Send + Sync,
{
    type Rejection = RestError;
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let path = <Path<(Id, Id)>>::from_request_parts(parts, state)
            .await
            .unwrap();
        Ok(path.into())
    }
}

pub struct SchemaPath {
    pub channel_id: Id,
    pub message_type_id: Id,
    pub schema_id: Id,
}

impl ResourcePath for SchemaPath {
    type Resource = Schema;

    type Handler = FindSchemaQueryHandler;

    type Query = FindSchemaQuery;

    fn query(&self) -> Self::Query {
        FindSchemaQueryBuilder::default()
            .message_type_id(self.message_type_id)
            .channel_id(self.channel_id)
            .version(self.schema_id)
            .build()
            .unwrap()
    }
}

#[async_trait::async_trait]
impl<S> FromRequestParts<S> for SchemaPath
where
    S: Send + Sync,
{
    type Rejection = RestError;
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let path = <Path<(Id, Id, Id)>>::from_request_parts(parts, state)
            .await
            .unwrap();
        Ok(path.into())
    }
}

impl From<Path<(Id, Id, Id)>> for SchemaPath {
    fn from(value: Path<(Id, Id, Id)>) -> Self {
        SchemaPath::from(value.0)
    }
}

impl From<(Id, Id, Id)> for SchemaPath {
    fn from((channel_id, message_type_id, schema_id): (Id, Id, Id)) -> Self {
        SchemaPath {
            channel_id,
            message_type_id,
            schema_id,
        }
    }
}
