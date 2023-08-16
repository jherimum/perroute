use super::{business_unit::BusinessUnit, message_type::MessageType, template::Template};
use crate::{
    query::{ModelQueryBuilder, Projection},
    DatabaseModel,
};
use derive_builder::Builder;
use derive_getters::Getters;
use derive_setters::Setters;
use perroute_commons::types::{code::Code, id::Id, json_schema::JsonSchema, vars::Vars};
use perroute_connectors::types::DispatchType;
use serde::{Deserialize, Serialize};
use sqlx::{types::Json, FromRow, PgExecutor, QueryBuilder, Type};
use std::fmt::Display;

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
    business_unit_id: Option<Id>,

    #[builder(default)]
    bu_code: Option<Code>,

    #[builder(default)]
    version: Option<Version>,
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
                INNER JOIN business_units bu
                ON s.business_unit_id = bu.id
                WHERE 1=1 "#,
        );

        if let Some(message_type_code) = self.message_type_code.clone() {
            builder.push(" AND mt.code = ");
            builder.push_bind(message_type_code);
        }

        if let Some(bu_code) = self.bu_code.clone() {
            builder.push(" AND bu.code = ");
            builder.push_bind(bu_code);
        }

        if let Some(id) = self.id {
            builder.push(" AND s.id = ");
            builder.push_bind(id);
        }

        if let Some(message_type_id) = self.message_type_id {
            builder.push(" AND s.message_type_id = ");
            builder.push_bind(message_type_id);
        }

        if let Some(version) = self.version {
            builder.push(" AND s.version = ");
            builder.push_bind(version);
        }

        if let Some(business_unit_id) = self.business_unit_id {
            builder.push(" AND s.business_unit_id = ");
            builder.push_bind(business_unit_id);
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

    #[setters(skip)]
    message_type_id: Id,

    #[setters(skip)]
    business_unit_id: Id,

    #[setters(skip)]
    version: Version,

    enabled: bool,
    vars: Vars,
    published: bool,
    value: JsonSchema,
}

impl Schema {
    pub async fn active_template<'e, E: PgExecutor<'e>>(
        &self,
        exec: E,
        dispatch_type: DispatchType,
    ) -> Result<Option<Template>, sqlx::Error> {
        todo!()
    }

    pub async fn templates<'e, E: PgExecutor<'e>>(
        &self,
        exec: E,
    ) -> Result<Vec<Template>, sqlx::Error> {
        todo!()
    }

    pub async fn message_type<'e, E: PgExecutor<'e>>(
        &self,
        exec: E,
    ) -> Result<MessageType, sqlx::Error> {
        todo!()
    }

    pub async fn business_unit<'e, E: PgExecutor<'e>>(
        &self,
        exec: E,
    ) -> Result<BusinessUnit, sqlx::Error> {
        todo!()
    }

    pub async fn save<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<Self, sqlx::Error> {
        sqlx::query_as(
            r#"
                INSERT INTO schemas (id, value, version, published, message_type_id, enabled, vars, business_unit_id) 
                VALUES($1, $2, $3, $4, $5, $6, $7) RETURNING *
            "#,
        )
        .bind(self.id)
        .bind(self.value)
        .bind(self.version)
        .bind(self.published)
        .bind(self.message_type_id)
        .bind(self.enabled)
        .bind(self.vars)
        .bind(self.business_unit_id)
        .fetch_one(exec)
        .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<Self, sqlx::Error> {
        sqlx::query_as(
            r#"
            UPDATE schemas 
            SET 
                value= $2, 
                published= $3,
                enabled = $4,
                vars =$5

            WHERE id= $1 RETURNING *
            "#,
        )
        .bind(self.id)
        .bind(self.value)
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
            SELECT coalesce(MAX(version),0) as version
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
