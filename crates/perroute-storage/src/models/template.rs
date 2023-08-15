use crate::{
    log_query_error,
    query::{ModelQueryBuilder, Projection},
    DatabaseModel,
};
use derive_builder::Builder;
use derive_getters::Getters;
use derive_setters::Setters;
use perroute_commons::types::{id::Id, template::TemplateSnippet, vars::Vars};
use perroute_connectors::types::DispatchType;
use sqlx::{types::Json, FromRow, PgExecutor, QueryBuilder};
use tap::TapFallible;

use super::{business_unit::BusinessUnit, message_type::MessageType, schema::Schema};

impl DatabaseModel for Template {}

#[derive(Debug, Builder)]
pub struct TemplatesQuery {
    #[builder(default)]
    id: Option<Id>,

    #[builder(default)]
    message_type_id: Option<Id>,

    #[builder(default)]
    bu_id: Option<Id>,
}

impl ModelQueryBuilder<Template> for TemplatesQuery {
    fn build(&self, projection: Projection) -> QueryBuilder<'_, sqlx::Postgres> {
        let mut builder = projection.query_builder();

        builder.push(" FROM templates WHERE 1=1 ");

        if let Some(id) = self.id {
            builder.push(" AND id = ");
            builder.push_bind(id);
        }

        if let Some(bu_id) = self.bu_id {
            builder.push(" AND bu_id = ");
            builder.push_bind(bu_id);
        }

        if let Some(message_type_id) = self.message_type_id {
            builder.push(" AND message_type_id = ");
            builder.push_bind(message_type_id);
        }

        builder
    }
}

#[derive(Debug, FromRow, PartialEq, Eq, Clone, Getters, Setters, Builder)]
#[builder(setter(into))]
#[setters(prefix = "set_")]
#[setters(into)]
pub struct Template {
    #[setters(skip)]
    id: Id,

    #[setters(skip)]
    dispatch_type: DispatchType,

    subject: Option<String>,
    text: Option<TemplateSnippet>,
    html: Option<TemplateSnippet>,
    vars: Json<Vars>,
    active: bool,

    #[setters(skip)]
    schema_id: Id,

    #[setters(skip)]
    message_type_id: Id,

    #[setters(skip)]
    bu_id: Id,
}

impl Template {
    pub async fn message_type<'e, E: PgExecutor<'e>>(
        &self,
        exec: E,
    ) -> Result<MessageType, sqlx::Error> {
        todo!()
    }

    pub async fn schema<'e, E: PgExecutor<'e>>(&self, exec: E) -> Result<Schema, sqlx::Error> {
        todo!()
    }

    pub async fn bu<'e, E: PgExecutor<'e>>(&self, exec: E) -> Result<BusinessUnit, sqlx::Error> {
        todo!()
    }

    pub async fn save<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<Self, sqlx::Error> {
        sqlx::query_as(
            r#"
        INSERT INTO templates (id, dispatch_type, subject, text, html, vars, active, schema_id, message_type_id, bu_id) 
        VALUES($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        RETURNING *"#,
        )
        .bind(self.id)
        .bind(self.dispatch_type)
        .bind(self.subject)
        .bind(self.text)
        .bind(self.html)
        .bind(self.vars)
        .bind(self.active)
        .bind(self.schema_id)
        .bind(self.message_type_id)
        .bind(self.bu_id)
        
        
        .fetch_one(exec)
        .await
        .tap_err(log_query_error!())
    }

    pub async fn update<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<Self, sqlx::Error> {
        sqlx::query_as(
            r#"
            UPDATE templates 
            SET subject= $2, text=$3, html=$4, vars=$5, active=$6
            WHERE id=$1 
            RETURNING *"#,
        )
        .bind(self.id)
        .bind(self.subject)
        .bind(self.text)
        .bind(self.html)
        .bind(self.vars)
        .bind(self.active)
        .fetch_one(exec)
        .await
        .tap_err(log_query_error!())
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<bool, sqlx::Error> {
        sqlx::query("DELETE FROM templates WHERE id= $1")
            .bind(self.id)
            .execute(exec)
            .await
            .tap_err(log_query_error!())
            .map(|result| result.rows_affected() > 0)
    }
}
