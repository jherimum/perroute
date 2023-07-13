use chrono::NaiveDateTime;
use derive_builder::Builder;
use derive_getters::Getters;
use perroute_commons::types::id::Id;
use serde::Serialize;
use sqlx::{FromRow, QueryBuilder};

use crate::query::{ModelQueryBuilder, Projection};

#[derive(Debug, Default, Builder)]
#[builder(default)]
pub struct ApiKeyQuery {
    id: Option<Id>,
    channel_id: Option<Id>,
}

impl ModelQueryBuilder<ApiKey> for ApiKeyQuery {
    fn build(&self, _: Projection) -> QueryBuilder<'_, sqlx::Postgres> {
        todo!()
    }
}

#[derive(Clone, Serialize, Debug, Default, Getters, FromRow, Builder)]
#[builder(setter(into))]
pub struct ApiKey {
    id: Id,
    channel_id: uuid::Uuid,
    name: String,
    prefix: String,
    hash: String,
    expires_at: Option<NaiveDateTime>,
    created_at: NaiveDateTime,
}
