use bon::Builder;
use derive_getters::Getters;
use perroute_commons::types::{id::Id, ProviderId, Timestamp};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, types::Json};

#[derive(
    Debug, Clone, PartialEq, Eq, Getters, Builder, Serialize, Deserialize,
)]
pub struct DispatcherError {
    pub code: String,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Eq, FromRow, Getters)]
pub struct DispatcherLog {
    id: Id,
    message_id: Id,
    provider_id: ProviderId,
    success: bool,
    error: Option<Json<DispatcherError>>,
    created_at: Timestamp,
}

impl DispatcherLog {
    pub fn build_success(message_id: Id, provider_id: ProviderId) -> Self {
        DispatcherLog {
            id: Id::new(),
            message_id,
            provider_id,
            success: true,
            error: None,
            created_at: Default::default(),
        }
    }

    pub fn build_error(
        message_id: Id,
        provider_id: ProviderId,
        error: DispatcherError,
    ) -> Self {
        DispatcherLog {
            id: Id::new(),
            message_id,
            provider_id,
            success: false,
            error: Some(Json(error)),
            created_at: Default::default(),
        }
    }
}
