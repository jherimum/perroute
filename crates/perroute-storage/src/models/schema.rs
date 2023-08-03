use std::fmt::Display;

use crate::{
    query::{ModelQueryBuilder, Projection},
    DatabaseModel,
};
use derive_builder::Builder;
use derive_getters::Getters;
use derive_setters::Setters;
use perroute_commons::types::{code::Code, id::Id, json_schema::JsonSchema, vars::Vars};
use serde::{Deserialize, Serialize};
use sqlx::{types::Json, FromRow, PgExecutor, QueryBuilder, Type};

use super::{channel::Channel, message_type::MessageType};

impl DatabaseModel for Schema {}

#[derive(Debug, Default, Builder)]
pub struct SchemasQuery {
    #[builder(default)]
    id: Option<Id>,
    #[builder(default)]
    message_type_id: Option<Id>,
    #[builder(default)]
    message_type_code: Option<Code>,
    #[builder(default)]
    channel_id: Option<Id>,
    #[builder(default)]
    version: Option<Version>,
    #[builder(default)]
    channel_code: Option<Code>,
}

impl ModelQueryBuilder<Schema> for SchemasQuery {
    fn build(&self, projection: Projection) -> sqlx::QueryBuilder<'_, sqlx::Postgres> {
        let mut builder = QueryBuilder::new(match projection {
            Projection::Row => "SELECT s.*",
            Projection::Count => "SELECT COUNT(*)",
            Projection::Id => "SELECT s.id",
        });

        builder.push(
            r#" 
                FROM schemas s 
                INNER JOIN message_types mt 
                ON s.message_type_id = mt.id 
                INNER JOIN channels c
                ON s.channel_id = c.id
                WHERE 1=1 "#,
        );

        if let Some(channel_code) = self.channel_code.clone() {
            builder.push(" AND c.code = ");
            builder.push_bind(channel_code);
        }

        if let Some(message_type_code) = self.message_type_code.clone() {
            builder.push(" AND mt.code = ");
            builder.push_bind(message_type_code);
        }

        if let Some(id) = self.id {
            builder.push(" AND s.id = ");
            builder.push_bind(id);
        }

        if let Some(message_type_id) = self.message_type_id {
            builder.push(" AND s.message_type_id = ");
            builder.push_bind(message_type_id);
        }

        if let Some(channel_id) = self.channel_id {
            builder.push(" AND s.channel_id = ");
            builder.push_bind(channel_id);
        }

        if let Some(version) = self.version {
            builder.push(" AND s.version = ");
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

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
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

    enabled: bool,

    #[setters(skip)]
    #[getter(skip)]
    vars: Json<Vars>,
}

impl Schema {
    pub fn vars(&self) -> &Vars {
        &self.vars
    }

    pub fn set_vars(mut self, vars: Vars) -> Self {
        self.vars = Json(vars);
        self
    }

    pub async fn message_type<'e, E: PgExecutor<'e>>(
        &self,
        exec: E,
    ) -> Result<MessageType, sqlx::Error> {
        todo!()
    }

    pub async fn channel<'e, E: PgExecutor<'e>>(&self, exec: E) -> Result<Channel, sqlx::Error> {
        todo!()
    }

    pub async fn save<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<Self, sqlx::Error> {
        sqlx::query_as(
            r#"
                INSERT INTO schemas (id, schema, version, published, message_type_id, channel_id, enabled) 
                VALUES($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *
            "#,
        )
        .bind(self.id)
        .bind(self.schema)
        .bind(self.version)
        .bind(self.published)
        .bind(self.message_type_id)
        .bind(self.channel_id)
        .bind(self.enabled)
        .bind(self.vars)
        .fetch_one(exec)
        .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<Self, sqlx::Error> {
        sqlx::query_as(
            r#"
            UPDATE schemas 
            SET 
                schema= $2, 
                published= $3,
                enabled = $4,
                vars =$5

            WHERE id= $1 RETURNING *
            "#,
        )
        .bind(self.id)
        .bind(self.schema)
        .bind(self.published)
        .bind(self.enabled)
        .bind(self.vars)
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
