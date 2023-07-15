use crate::{
    query::{ModelQueryBuilder, Projection},
    DatabaseModel,
};
use chrono::NaiveDateTime;
use derive_builder::Builder;
use derive_getters::Getters;
use derive_setters::Setters;
use perroute_commons::types::id::Id;
use serde::Serialize;
use sqlx::{FromRow, PgExecutor, QueryBuilder};

#[derive(Debug, Default, Builder)]
#[builder(default)]
pub struct ApiKeyQuery {
    #[builder(default)]
    id: Option<Id>,
    #[builder(default)]
    channel_id: Option<Id>,
    #[builder(default)]
    hash: Option<String>,
}

impl ModelQueryBuilder<ApiKey> for ApiKeyQuery {
    fn build(&self, projection: Projection) -> QueryBuilder<'_, sqlx::Postgres> {
        dbg!(self);

        let mut builder = projection.query_builder();
        builder.push(" FROM api_keys WHERE 1=1");

        if let Some(id) = &self.id {
            builder.push(" AND id = ");
            builder.push_bind(id);
        }

        if let Some(channel_id) = &self.channel_id {
            builder.push(" AND channel_id = ");
            builder.push_bind(channel_id);
        }

        if let Some(hash) = &self.hash {
            builder.push(" AND hash = ");
            builder.push_bind(hash);
        }

        dbg!(builder.sql());

        builder
    }
}

impl DatabaseModel for ApiKey {}

#[derive(Clone, Serialize, Debug, Default, Getters, FromRow, Builder, Setters)]
#[builder(setter(into))]
#[setters(prefix = "set_")]
#[setters(into)]
pub struct ApiKey {
    #[setters(skip)]
    id: Id,

    #[setters(skip)]
    channel_id: Id,

    #[setters(skip)]
    name: String,

    #[setters(skip)]
    prefix: String,

    #[setters(skip)]
    hash: String,

    #[setters(skip)]
    created_at: NaiveDateTime,

    #[setters(skip)]
    expires_at: Option<NaiveDateTime>,

    #[builder(default)]
    revoked_at: Option<NaiveDateTime>,
}

impl ApiKey {
    pub fn revoked(&self) -> bool {
        self.revoked_at.is_some()
    }

    pub async fn save<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<Self, sqlx::Error> {
        sqlx::query_as(r#"
                            INSERT INTO api_keys (id, channel_id, name, prefix, hash, created_at, expires_at) 
                            VALUES($1, $2, $3, $4, $5, $6, $7) RETURNING *
                            "#)
            .bind(self.id)
            .bind(self.channel_id)
            .bind(self.name)
            .bind(self.prefix)
            .bind(self.hash)
            .bind(self.created_at)
            .bind(self.expires_at)
            .fetch_one(exec)
            .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<Self, sqlx::Error> {
        sqlx::query_as(
            r#"
                    UPDATE api_keys SET revoked_at = $2
                    WHERE id = $1
                    RETURNING *
                "#,
        )
        .bind(self.id)
        .bind(self.revoked_at)
        .fetch_one(exec)
        .await
    }
}
