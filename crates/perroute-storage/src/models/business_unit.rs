use super::{channel::Channel, message_type::MessageType};
use crate::{
    log_query_error,
    query::{ModelQueryBuilder, Projection},
    DatabaseModel,
};
use derive_builder::Builder;
use derive_getters::Getters;
use derive_setters::Setters;
use perroute_commons::types::{code::Code, id::Id, vars::Vars};
use sqlx::{types::Json, FromRow, PgExecutor};
use tap::TapFallible;

#[derive(Debug, Default, Builder)]
#[builder(default)]
pub struct BusinessUnitQuery {
    id: Option<Id>,
    code: Option<Code>,
}

impl ModelQueryBuilder<BusinessUnit> for BusinessUnitQuery {
    fn build(&self, projection: Projection) -> sqlx::QueryBuilder<'_, sqlx::Postgres> {
        let mut builder = projection.query_builder();

        builder.push(" FROM business_units where 1=1");

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

impl DatabaseModel for BusinessUnit {}

#[derive(Debug, FromRow, PartialEq, Eq, Clone, Getters, Setters, Builder)]
#[builder(setter(into))]
#[setters(prefix = "set_")]
#[setters(into)]
pub struct BusinessUnit {
    #[setters(skip)]
    id: Id,
    #[setters(skip)]
    code: Code,
    name: String,
    vars: Json<Vars>,
}

impl BusinessUnit {
    #[tracing::instrument(name = "business_unit.save", skip(exec))]
    pub async fn save<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<Self, sqlx::Error> {
        sqlx::query_as(
            "INSERT INTO business_units (id, code, name, vars) VALUES($1, $2, $3, $4) RETURNING *",
        )
        .bind(self.id)
        .bind(self.code)
        .bind(self.name)
        .bind(self.vars)
        .fetch_one(exec)
        .await
        .tap_err(log_query_error!())
    }

    #[tracing::instrument(name = "business_unit.update", skip(exec))]
    pub async fn update<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<Self, sqlx::Error> {
        sqlx::query_as("UPDATE business_units SET name= $2, vars=$3 WHERE id= $1 RETURNING *")
            .bind(self.id)
            .bind(self.name)
            .bind(self.vars)
            .fetch_one(exec)
            .await
            .tap_err(log_query_error!())
    }

    #[tracing::instrument(name = "business_unit.delete", skip(exec))]
    pub async fn delete<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<bool, sqlx::Error> {
        sqlx::query("DELETE FROM business_units WHERE id= $1")
            .bind(self.id)
            .execute(exec)
            .await
            .tap_err(log_query_error!())
            .map(|result| result.rows_affected() > 0)
    }

    pub async fn message_types<'e, E: PgExecutor<'e>>(
        self,
        exec: E,
    ) -> Result<Vec<MessageType>, sqlx::Error> {
        todo!()
    }

    pub async fn channels<'e, E: PgExecutor<'e>>(
        self,
        exec: E,
    ) -> Result<Vec<Channel>, sqlx::Error> {
        todo!()
    }
}
