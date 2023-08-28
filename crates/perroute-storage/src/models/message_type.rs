use super::{
    business_unit::{BusinessUnit, BusinessUnitQueryBuilder},
    schema::{Schema, SchemasQuery, SchemasQueryBuilder},
};
use crate::{
    error::StorageError,
    log_query_error,
    query::{FetchableModel, ModelQueryBuilder, Projection},
    DatabaseModel,
};
use derive_builder::Builder;
use derive_getters::Getters;
use derive_setters::Setters;
use perroute_commons::types::{code::Code, id::Id, vars::Vars};
use sqlx::{FromRow, PgExecutor, Postgres, QueryBuilder};
use tap::TapFallible;

impl DatabaseModel for MessageType {}

#[derive(Debug, Default, Builder)]
#[builder(default)]
pub struct MessageTypeQuery {
    id: Option<Id>,
    code: Option<Code>,
    business_unit_id: Option<Id>,
}

impl MessageTypeQuery {
    pub fn with_business_unit(business_unit_id: Id) -> Self {
        Self {
            business_unit_id: Some(business_unit_id),
            ..Default::default()
        }
    }

    pub fn with_id(id: Id) -> Self {
        Self {
            id: Some(id),
            ..Default::default()
        }
    }

    pub fn with_code_and_business_unit(code: Code, business_unit_id: Id) -> Self {
        Self {
            code: Some(code),
            business_unit_id: Some(business_unit_id),
            ..Default::default()
        }
    }
}

impl ModelQueryBuilder<MessageType> for MessageTypeQuery {
    fn build(&self, projection: Projection) -> QueryBuilder<'_, Postgres> {
        let mut query_builder = projection.query_builder();

        query_builder.push(" FROM message_types WHERE 1=1");

        if let Some(code) = self.code.clone() {
            query_builder.push(" and code = ");
            query_builder.push_bind(code);
        }

        if let Some(id) = self.id {
            query_builder.push(" and id = ");
            query_builder.push_bind(id);
        }

        if let Some(business_unit_id) = self.business_unit_id {
            query_builder.push(" and business_unit_id = ");
            query_builder.push_bind(business_unit_id);
        }

        query_builder
    }
}

#[derive(Debug, FromRow, PartialEq, Eq, Clone, Getters, Setters, Builder)]
#[builder(setter(into))]
#[setters(prefix = "set_")]
#[setters(into)]
pub struct MessageType {
    #[setters(skip)]
    id: Id,

    #[setters(skip)]
    code: Code,

    name: String,
    enabled: bool,
    vars: Vars,

    #[setters(skip)]
    business_unit_id: Id,
}

impl MessageType {
    pub async fn business_unit<'e, E: PgExecutor<'e>>(
        &self,
        exec: E,
    ) -> Result<BusinessUnit, StorageError> {
        BusinessUnit::find_one(
            exec,
            BusinessUnitQueryBuilder::default()
                .id(Some(self.business_unit_id))
                .build()
                .unwrap(),
        )
        .await
    }

    pub async fn exists_schemas<'e, E: PgExecutor<'e>>(
        &self,
        exec: E,
    ) -> Result<bool, StorageError> {
        Schema::exists(exec, SchemasQuery::with_message_type_id(self.id)).await
    }

    pub async fn schemas<'e, E: PgExecutor<'e>>(
        self,
        exec: E,
    ) -> Result<Vec<Schema>, StorageError> {
        Schema::query(
            exec,
            SchemasQueryBuilder::default()
                .message_type_id(Some(self.id))
                .build()
                .unwrap(),
        )
        .await
    }

    pub async fn save<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<Self, StorageError> {
        Ok(sqlx::query_as(
            r#"
                    INSERT INTO message_types (id, code, name, enabled, vars, business_unit_id) 
                    VALUES($1, $2, $3, $4, $5, $6) RETURNING *
                "#,
        )
        .bind(self.id)
        .bind(self.code)
        .bind(self.name)
        .bind(self.enabled)
        .bind(self.vars)
        .bind(self.business_unit_id)
        .fetch_one(exec)
        .await
        .tap_err(log_query_error!())?)
    }

    pub async fn update<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<Self, StorageError> {
        Ok(sqlx::query_as(
            r#"
                    UPDATE message_types 
                    SET name= $2, enabled= $3, vars= $4
                    WHERE id= $1 RETURNING *
                "#,
        )
        .bind(self.id)
        .bind(self.name)
        .bind(self.enabled)
        .bind(self.vars)
        .fetch_one(exec)
        .await
        .tap_err(log_query_error!())?)
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<bool, StorageError> {
        Ok(sqlx::query("DELETE FROM message_types WHERE id= $1")
            .bind(self.id)
            .execute(exec)
            .await
            .tap_err(log_query_error!())
            .map(|result| result.rows_affected() > 0)?)
    }
}
