use crate::query::{ModelQuery, ModelQueryFetch, Projection};
use derive_builder::Builder;
use derive_getters::Getters;
use derive_setters::Setters;
use perroute_commons::types::{id::Id, json_schema::JsonSchema};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgExecutor, QueryBuilder, Type};

#[derive(Debug, Default, Builder)]
pub struct SchemasQuery {
    #[builder(default)]
    id: Option<Id>,
    #[builder(default)]
    message_type_id: Option<Id>,
    #[builder(default)]
    channel_id: Option<Id>,

    #[builder(default)]
    version: Option<Version>,
}

impl SchemasQuery {
    pub fn by_id(id: Id) -> Self {
        Self {
            id: Some(id),
            ..Default::default()
        }
    }

    pub fn by_message_type_and_id(id: Id, message_type_id: Id) -> Self {
        Self {
            id: Some(id),
            message_type_id: Some(message_type_id),
            ..Default::default()
        }
    }

    pub fn by_message_type(message_type_id: Id) -> Self {
        Self {
            message_type_id: Some(message_type_id),
            ..Default::default()
        }
    }
}

impl ModelQuery<Schema> for SchemasQuery {
    fn query_builder(&self, projection: Projection) -> sqlx::QueryBuilder<'_, sqlx::Postgres> {
        let mut builder = QueryBuilder::new({
            match projection {
                Projection::Row => "SELECT *",
                Projection::Count => "SELECT COUNT(*)",
                Projection::Id => "SELECT id",
            }
        });

        builder.push(" FROM schemas WHERE 1=1");

        if let Some(id) = self.id {
            builder.push(" AND id = ");
            builder.push_bind(id);
        }

        if let Some(message_type_id) = self.message_type_id {
            builder.push(" AND message_type_id = ");
            builder.push_bind(message_type_id);
        }

        if let Some(channel_id) = self.channel_id {
            builder.push(" AND channel_id = ");
            builder.push_bind(channel_id);
        }

        if let Some(version) = self.version {
            builder.push(" AND version = ");
            builder.push_bind(version);
        }

        builder
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Type, Serialize, Deserialize, Copy)]
#[sqlx(transparent)]
#[serde(transparent)]
pub struct Version(i32);

impl Default for Version {
    fn default() -> Self {
        Self(1)
    }
}

impl Version {
    pub const fn increment(self) -> Self {
        Self(self.0 + 1)
    }
}

impl From<i32> for Version {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl From<Version> for i32 {
    fn from(value: Version) -> Self {
        value.0
    }
}

impl From<&Version> for i32 {
    fn from(value: &Version) -> Self {
        value.0
    }
}

#[derive(Debug, FromRow, PartialEq, Eq, Clone, Getters, Setters, Builder)]
#[builder(setter(into))]
#[setters(prefix = "set_")]
#[setters(into)]
pub struct Schema {
    #[setters(skip)]
    id: Id,
    schema: JsonSchema,
    #[setters(skip)]
    version: Version,
    published: bool,
    #[setters(skip)]
    message_type_id: Id,
    #[setters(skip)]
    channel_id: Id,
}

impl Schema {
    pub async fn count<'e, E: PgExecutor<'e>>(
        exec: E,
        query: SchemasQuery,
    ) -> Result<i64, sqlx::Error> {
        query.count(exec).await
    }

    pub async fn query<'e, E: PgExecutor<'e>>(
        exec: E,
        query: SchemasQuery,
    ) -> Result<Vec<Schema>, sqlx::Error> {
        query.many(exec).await
    }

    pub async fn find<'e, E: PgExecutor<'e>>(
        exec: E,
        query: SchemasQuery,
    ) -> Result<Option<Schema>, sqlx::Error> {
        query.one(exec).await
    }

    pub async fn save<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<Self, sqlx::Error> {
        sqlx::query_as(
            r#"
                INSERT INTO schemas (id, schema, version, published, message_type_id, channel_id) 
                VALUES($1, $2, $3, $4, $5, $6) RETURNING *
            "#,
        )
        .bind(self.id)
        .bind(self.schema)
        .bind(self.version)
        .bind(self.published)
        .bind(self.message_type_id)
        .bind(self.channel_id)
        .fetch_one(exec)
        .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<Self, sqlx::Error> {
        sqlx::query_as(
            r#"
            UPDATE schemas 
            SET 
                schema= $1, 
                published= $2
            WHERE id= $3 RETURNING *
            "#,
        )
        .bind(self.schema)
        .bind(self.published)
        .bind(self.id)
        .fetch_one(exec)
        .await
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<bool, sqlx::Error> {
        sqlx::query("DELETE FROM schemas WHERE id= $1")
            .bind(self.id)
            .execute(exec)
            .await
            .map(|r| r.rows_affected() > 0)
    }

    pub async fn max_version_number(
        exec: &mut sqlx::PgConnection,
        message_type_id: &Id,
    ) -> Result<Version, sqlx::Error> {
        sqlx::query_scalar::<_, Version>(
            r#"
            SELECT MAX(version) as version
            FROM schemas 
            WHERE 
                message_type_id= $1
            "#,
        )
        .bind(message_type_id)
        .fetch_optional(exec)
        .await
        .map(|r| r.unwrap_or_default())
    }
}
