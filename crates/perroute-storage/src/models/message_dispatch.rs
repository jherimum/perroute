use bon::{builder, Builder};
use derive_getters::Getters;
use perroute_commons::types::{id::Id, ProviderId, Timestamp};
use sqlx::prelude::FromRow;

#[derive(Debug, Clone, PartialEq, Eq, FromRow, Getters, Builder)]
pub struct MessageDispatch {
    #[builder(default)]
    id: Id,
    message_id: Id,
    provider_id: ProviderId,
    success: bool,
    created_at: Timestamp,
}
