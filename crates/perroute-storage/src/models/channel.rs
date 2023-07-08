use crate::{
    log_query_error,
    query::{ModelQuery, ModelQueryFetch},
};
use derive_builder::Builder;
use derive_getters::Getters;
use derive_setters::Setters;
use perroute_commons::types::{code::Code, id::Id};
use sqlx::{FromRow, PgExecutor, QueryBuilder};
use tap::TapFallible;

pub struct ChannelsQuery {
    pub id: Option<Id>,
    pub code: Option<Code>,
}
impl ModelQuery<Channel> for ChannelsQuery {
    fn query_builder(&self, count: bool) -> sqlx::QueryBuilder<'_, sqlx::Postgres> {
        let mut builder = QueryBuilder::new({
            if count {
                "SELECT COUNT(*)"
            } else {
                "SELECT *"
            }
        });

        builder.push(" FROM channels where 1=1");

        if let Some(id) = self.id {
            builder.push(" AND id = ");
            builder.push_bind(id);
        }

        if let Some(code) = self.code.clone() {
            builder.push(" AND code = ");
            builder.push_bind(code);
        }

        builder
    }
}

#[async_trait::async_trait]
impl ModelQueryFetch<Channel> for ChannelsQuery {}

#[derive(Debug, FromRow, PartialEq, Eq, Clone, Getters, Setters, Builder)]
#[builder(setter(into))]
#[setters(prefix = "set_")]
#[setters(into)]
pub struct Channel {
    #[setters(skip)]
    id: Id,
    #[setters(skip)]
    code: Code,
    name: String,
}

impl Channel {
    #[tracing::instrument(name = "channel.exists_by_code", skip(exec))]
    pub async fn exists_by_code<'e, E: PgExecutor<'e>>(
        exec: E,
        code: &Code,
    ) -> Result<bool, sqlx::Error> {
        ChannelsQuery {
            id: None,
            code: Some(code.clone()),
        }
        .count(exec)
        .await
        .tap_err(log_query_error!())
        .map(|result| result > 0)
    }

    #[tracing::instrument(name = "channel.save", skip(exec))]
    pub async fn save<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<Self, sqlx::Error> {
        sqlx::query_as("INSERT INTO channels (id, code, name) VALUES($1, $2, $3) RETURNING *")
            .bind(self.id)
            .bind(self.code)
            .bind(self.name)
            .fetch_one(exec)
            .await
            .tap_err(log_query_error!())
    }

    #[tracing::instrument(name = "channel.update", skip(exec))]
    pub async fn update<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<Self, sqlx::Error> {
        sqlx::query_as("UPDATE channels SET name= $1 WHERE id= $2 RETURNING *")
            .bind(self.name)
            .bind(self.id)
            .fetch_one(exec)
            .await
            .tap_err(log_query_error!())
    }

    #[tracing::instrument(name = "channel.delete", skip(exec))]
    pub async fn delete<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<bool, sqlx::Error> {
        sqlx::query("DELETE FROM channels WHERE id= $1")
            .bind(self.id)
            .execute(exec)
            .await
            .tap_err(log_query_error!())
            .map(|result| result.rows_affected() > 0)
    }

    #[tracing::instrument(name = "channel.find_by_id", skip(exec))]
    pub async fn find_by_id<'e, E: PgExecutor<'e>>(
        exec: E,
        id: Id,
    ) -> Result<Option<Self>, sqlx::Error> {
        ChannelsQuery {
            id: Some(id),
            code: None,
        }
        .one(exec)
        .await
        .tap_err(log_query_error!())
    }

    #[tracing::instrument(name = "channel.find_by_code", skip(exec))]
    pub async fn find_by_code<'e, E: PgExecutor<'e>>(
        exec: E,
        code: &Code,
    ) -> Result<Option<Self>, sqlx::Error> {
        ChannelsQuery {
            id: None,
            code: Some(code.clone()),
        }
        .one(exec)
        .await
        .tap_err(log_query_error!())
    }

    pub async fn query<'e, E: PgExecutor<'e>>(exec: E) -> Result<Vec<Self>, sqlx::Error> {
        ChannelsQuery {
            id: None,
            code: None,
        }
        .many(exec)
        .await
        .tap_err(log_query_error!())
    }
}
