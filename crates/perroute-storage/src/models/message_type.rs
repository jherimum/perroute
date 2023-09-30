use crate::{
    log_query_error,
    query::{FetchableModel, ModelQueryBuilder, Projection},
    DatabaseModel, Result,
};
use derive_builder::Builder;
use derive_getters::Getters;
use derive_setters::Setters;
use perroute_commons::types::{code::Code, id::Id, json_schema::JsonSchema, vars::Vars};
use sqlx::{FromRow, PgExecutor, Postgres, QueryBuilder};
use tap::TapFallible;

use super::route::{Route, RouteQuery};

impl DatabaseModel for MessageType {}

#[derive(Debug, Default, Builder)]
#[builder(default)]
pub struct MessageTypeQuery {
    id: Option<Id>,
    code: Option<Code>,
    enabled: Option<bool>,
}

impl MessageTypeQuery {
    pub fn with_id(id: Id) -> Self {
        Self {
            id: Some(id),
            ..Default::default()
        }
    }

    pub fn with_code(code: Code) -> Self {
        Self {
            code: Some(code),
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

        if let Some(enabled) = self.enabled {
            query_builder.push(" and enabled = ");
            query_builder.push_bind(enabled);
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
    vars: Vars,
    schema: JsonSchema,
}

impl MessageType {
    pub async fn save<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<Self> {
        Ok(sqlx::query_as(
            r#"
                    INSERT INTO message_types 
                    (
                        id, 
                        code, 
                        name, 
                        vars, 
                        schema) 
                    VALUES($1, $2, $3, $4, $5, $6, $7) 
                    RETURNING *
                "#,
        )
        .bind(self.id)
        .bind(self.code)
        .bind(self.name)
        .bind(self.vars)
        .bind(self.schema)
        .fetch_one(exec)
        .await
        .tap_err(log_query_error!())?)
    }

    pub async fn update<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<Self> {
        Ok(sqlx::query_as(
            r#"
                    UPDATE message_types 
                    SET 
                        name= $2, 
                        vars= $3,
                        schema = $4
                    WHERE 
                        id= $1 
                    RETURNING *
                "#,
        )
        .bind(self.id)
        .bind(self.name)
        .bind(self.vars)
        .bind(self.schema)
        .fetch_one(exec)
        .await
        .tap_err(log_query_error!())?)
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<bool> {
        Ok(sqlx::query("DELETE FROM message_types WHERE id= $1")
            .bind(self.id)
            .execute(exec)
            .await
            .tap_err(log_query_error!())
            .map(|result| result.rows_affected() > 0)?)
    }

    pub async fn routes<'e, E: PgExecutor<'e>>(&self, exec: E) -> Result<Vec<Route>> {
        Route::query(exec, RouteQuery::with_message_type_id(self.id)).await
    }
}
