use derive_builder::Builder;
use derive_getters::Getters;
use derive_setters::Setters;
use perroute_commons::types::{id::Id, json_schema::JsonSchema};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgExecutor, Type};

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
}

impl Schema {
    pub async fn save<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<Self, sqlx::Error> {
        sqlx::query_as(
            r#"
                INSERT INTO schemas (id, schema, version, published, message_type_id) 
                VALUES($1, $2, $3, $4, $5) RETURNING *
            "#,
        )
        .bind(self.id)
        .bind(self.schema)
        .bind(self.version)
        .bind(self.published)
        .bind(self.message_type_id)
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

    pub async fn find_by_id<'e, E: PgExecutor<'e>>(
        exec: E,
        id: &Id,
    ) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as(
            r#"
            SELECT * 
            FROM schemas 
            WHERE id= $1
            "#,
        )
        .bind(id)
        .fetch_optional(exec)
        .await
    }

    pub async fn find_message_type_id_and_id<'e, E: PgExecutor<'e>>(
        exec: E,
        message_type_id: &Id,
        id: &Id,
    ) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as(
            r#"
            SELECT * 
            FROM schemas 
            WHERE 
                message_type_id= $1 
                AND id= $2
            "#,
        )
        .bind(message_type_id)
        .bind(id)
        .fetch_optional(exec)
        .await
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

    pub async fn query<'e, E: PgExecutor<'e>>(
        exec: E,
        message_type_id: &Id,
    ) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as(
            r#"
            SELECT * 
            FROM schemas 
            WHERE 
                message_type_id= $1
            "#,
        )
        .bind(message_type_id)
        .fetch_all(exec)
        .await
    }
}
