use crate::{
    log_query_error,
    query::{ModelQueryBuilder, Projection},
    DatabaseModel,
};
use derive_builder::Builder;
use derive_getters::Getters;
use derive_setters::Setters;
use perroute_commons::types::{code::Code, id::Id};
use sqlx::{FromRow, PgExecutor};
use tap::TapFallible;

#[derive(Debug, Default, Builder)]
pub struct ChannelsQuery {
    #[builder(default)]
    id: Option<Id>,
    #[builder(default)]
    code: Option<Code>,
}

impl ChannelsQuery {
    pub fn all() -> Self {
        Default::default()
    }

    pub fn by_id(id: Id) -> Self {
        Self {
            id: Some(id),
            ..Default::default()
        }
    }

    pub fn by_code(code: Code) -> Self {
        Self {
            code: Some(code),
            ..Default::default()
        }
    }
}

impl ModelQueryBuilder<Channel> for ChannelsQuery {
    fn build(&self, projection: Projection) -> sqlx::QueryBuilder<'_, sqlx::Postgres> {
        let mut builder = projection.query_builder();

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

impl DatabaseModel for Channel {}

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
}
