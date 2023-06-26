use super::actor::ActorExtractor;
use crate::{errors::PerrouteBackofficeApiError, rest::Buses};
use async_trait::async_trait;
use axum::{
    extract::{FromRequestParts, Path},
    http::request::Parts,
};
use perroute_commons::{
    rest::RestError,
    types::{actor::Actor, id::Id},
};
use perroute_cqrs::query_bus::{
    bus::QueryBus, handlers::channel::find_channel::FindChannelQueryHandler,
    queries::FindChannelQueryBuilder,
};
use perroute_storage::models::channel::Channel;
use std::{marker::PhantomData, ops::Deref};

#[derive(Debug)]
pub struct ChannelExtractor<S> {
    channel: Channel,
    marker: PhantomData<S>,
}

impl<S> Deref for ChannelExtractor<S> {
    type Target = Channel;

    fn deref(&self) -> &Self::Target {
        &self.channel
    }
}

#[async_trait]
impl FromRequestParts<Buses> for ChannelExtractor<Path<Id>> {
    type Rejection = RestError;

    async fn from_request_parts(parts: &mut Parts, buses: &Buses) -> Result<Self, Self::Rejection> {
        let ActorExtractor(actor) = ActorExtractor::from_request_parts(parts, buses)
            .await
            .unwrap();
        let p = <Path<Id>>::from_request_parts(parts, buses).await.unwrap();
        let channel = check_channel(&buses.query_bus, &p.0, &actor).await?;

        Ok(ChannelExtractor {
            channel,
            marker: PhantomData,
        })
    }
}

#[async_trait]
impl FromRequestParts<Buses> for ChannelExtractor<Path<(Id, Id)>> {
    type Rejection = RestError;

    async fn from_request_parts(parts: &mut Parts, buses: &Buses) -> Result<Self, Self::Rejection> {
        let ActorExtractor(actor) = ActorExtractor::from_request_parts(parts, buses)
            .await
            .unwrap();
        let p = <Path<Id>>::from_request_parts(parts, buses).await.unwrap();
        let channel = check_channel(&buses.query_bus, &p.0, &actor).await?;

        Ok(ChannelExtractor {
            channel,
            marker: PhantomData,
        })
    }
}

async fn check_channel(
    query_bus: &QueryBus,
    channel_id: &Id,
    actor: &Actor,
) -> Result<Channel, RestError> {
    let query = FindChannelQueryBuilder::default()
        .channel_id(*channel_id)
        .build()
        .unwrap();

    query_bus
        .execute::<_, FindChannelQueryHandler, _>(actor, query)
        .await
        .map_err(PerrouteBackofficeApiError::from)?
        .ok_or(RestError::NotFound(format!(
            "Channel {channel_id} not found"
        )))
}
