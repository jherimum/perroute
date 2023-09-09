use std::{fmt::Debug, str::FromStr};

use derive_getters::Getters;
use perroute_commons::types::id::Id;
use serde::{Deserialize, Serialize};
use sqlx::{
    database::HasArguments,
    encode::IsNull,
    error::BoxDynError,
    postgres::{PgTypeInfo, PgValueFormat, PgValueRef},
    Decode, Encode, Postgres, Type,
};
use strum_macros::{Display, EnumString};

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct EventPublisherError(#[from] anyhow::Error);

#[async_trait::async_trait]
pub trait EventPublisher: Debug + Clone {
    async fn publish(&self, event: &Event) -> Result<(), EventPublisherError>;
}

pub trait IntoEvent {
    fn into_event(&self) -> Option<Event>;
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Display, EnumString)]
pub enum EventType {
    BusinessUnitCreated,
    MessageCreated,
    MessageDistributed,
}

impl Decode<'_, Postgres> for EventType {
    fn decode(value: PgValueRef<'_>) -> Result<Self, BoxDynError> {
        match value.format() {
            PgValueFormat::Binary => Ok(Self::from_str(value.as_str().unwrap()).unwrap()),
            PgValueFormat::Text => Ok(value.as_str()?.parse::<Self>()?),
        }
    }
}

impl<'q> Encode<'q, Postgres> for EventType {
    fn encode_by_ref(&self, buf: &mut <Postgres as HasArguments<'q>>::ArgumentBuffer) -> IsNull {
        buf.extend(self.to_string().as_str().as_bytes());
        IsNull::No
    }
}

impl Type<Postgres> for EventType {
    fn type_info() -> PgTypeInfo {
        <String as Type<Postgres>>::type_info()
    }
}

#[derive(Debug, Getters, Serialize, Deserialize, Clone)]
pub struct Event {
    entity_id: Id,
    ty: EventType,
}

impl Event {
    pub fn new(entity_id: Id, ty: EventType) -> Self {
        Self { entity_id, ty }
    }
}
