use std::{marker::PhantomData, ops::Deref};

use super::actor::ActorExtractor;
use crate::rest::Buses;
use async_trait::async_trait;
use axum::{
    extract::{FromRequestParts, Path},
    http::request::Parts,
};
use perroute_commons::{
    rest::RestError,
    types::{actor::Actor, id::Id},
};
use perroute_storage::models::message_type::MessageType;

#[derive(Debug)]
pub struct MessageTypeExtractor<S> {
    message_type: MessageType,
    marker: PhantomData<S>,
}

impl<S> Deref for MessageTypeExtractor<S> {
    type Target = MessageType;

    fn deref(&self) -> &Self::Target {
        &self.message_type
    }
}

#[async_trait]
impl FromRequestParts<Buses> for MessageTypeExtractor<Path<(Id, Id)>> {
    type Rejection = RestError;

    async fn from_request_parts(parts: &mut Parts, buses: &Buses) -> Result<Self, Self::Rejection> {
        let ActorExtractor(actor) = ActorExtractor::from_request_parts(parts, buses)
            .await
            .unwrap();
        let path = <Path<(Id, Id)>>::from_request_parts(parts, buses)
            .await
            .unwrap();

        Ok(MessageTypeExtractor {
            message_type: retrieve_message_type(&actor, path).await?,
            marker: PhantomData,
        })
    }
}

async fn retrieve_message_type(
    actor: &Actor,
    path: Path<(Id, Id)>,
) -> Result<MessageType, RestError> {
    todo!()
}
