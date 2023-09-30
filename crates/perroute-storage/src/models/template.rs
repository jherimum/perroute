use crate::{
    log_query_error,
    query::{ModelQueryBuilder, Projection},
    DatabaseModel, Result,
};
use derive_builder::Builder;
use derive_getters::Getters;
use derive_setters::Setters;
use perroute_commons::types::{id::Id, template::TemplateSnippet};
use sqlx::{FromRow, PgExecutor, QueryBuilder};
use std::ops::Deref;
use tap::TapFallible;

#[derive(Debug, Builder, Default)]
#[builder(default)]
pub struct TemplatesQuery {
    id: Option<Id>,
}

impl TemplatesQuery {
    pub fn with_id(id: Id) -> Self {
        Self {
            id: Some(id),
            ..Default::default()
        }
    }
}

impl ModelQueryBuilder<Template> for TemplatesQuery {
    fn build(&self, projection: Projection) -> QueryBuilder<'_, sqlx::Postgres> {
        let mut builder = projection.query_builder();

        builder.push(" FROM templates WHERE 1=1 ");

        if let Some(id) = self.id {
            builder.push(" AND id = ");
            builder.push_bind(id);
        }

        builder
    }
}

impl DatabaseModel for Template {}

#[derive(Debug, FromRow, PartialEq, Eq, Clone, Getters, Setters, Builder)]
#[builder(setter(into))]
#[setters(prefix = "set_")]
#[setters(into)]
pub struct Template {
    #[setters(skip)]
    id: Id,
    name: String,
    subject: Option<TemplateSnippet>,
    text: Option<TemplateSnippet>,
    html: Option<TemplateSnippet>,
}

impl Template {
    pub async fn save<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<Self> {
        Ok(sqlx::query_as(
            r#"
        INSERT INTO templates (
            id, 
            subject, 
            text, 
            html, 
            name) 
        VALUES($1, $2, $3, $4, $5)
        RETURNING *"#,
        )
        .bind(self.id)
        .bind(self.subject)
        .bind(self.text)
        .bind(self.html)
        .bind(self.name)
        .fetch_one(exec)
        .await
        .tap_err(log_query_error!())?)
    }

    pub async fn update<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<Self> {
        Ok(sqlx::query_as(
            r#"
            UPDATE templates 
            SET 
                subject= $2, 
                text=$3, 
                html=$4, 
                name=$5,
            WHERE id=$1 
            RETURNING *"#,
        )
        .bind(self.id)
        .bind(self.subject)
        .bind(self.text)
        .bind(self.html)
        .bind(self.name)
        .fetch_one(exec)
        .await
        .tap_err(log_query_error!())?)
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<bool> {
        Ok(sqlx::query("DELETE FROM templates WHERE id= $1")
            .bind(self.id)
            .execute(exec)
            .await
            .tap_err(log_query_error!())
            .map(|result| result.rows_affected() > 0)?)
    }

    pub async fn batch_delete<'e, E: PgExecutor<'e>>(ids: Vec<Id>, exec: E) -> Result<u64> {
        let uuids = ids.iter().map(|id| *id.deref()).collect::<Vec<_>>();
        Ok(sqlx::query("DELETE FROM templates WHERE id= ANY($1)")
            .bind(uuids)
            .execute(exec)
            .await
            .tap_err(log_query_error!())
            .map(|result| result.rows_affected())?)
    }
}
