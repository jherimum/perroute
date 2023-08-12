use crate::{
    log_query_error,
    query::{ModelQueryBuilder, Projection},
    DatabaseModel,
};
use derive_builder::Builder;
use derive_getters::Getters;
use derive_setters::Setters;
use perroute_commons::types::{id::Id, template::TemplateSnippet};
use sqlx::{FromRow, PgExecutor, QueryBuilder};
use tap::TapFallible;

impl DatabaseModel for Template {}

#[derive(Debug, Builder)]
pub struct TemplatesQuery {
    #[builder(default)]
    id: Option<Id>,
    #[builder(default)]
    schema_id: Option<Id>,
    #[builder(default)]
    message_type_id: Option<Id>,
    #[builder(default)]
    channel_id: Option<Id>,
}

impl ModelQueryBuilder<Template> for TemplatesQuery {
    fn build(&self, projection: Projection) -> QueryBuilder<'_, sqlx::Postgres> {
        let mut builder = projection.query_builder();

        builder.push(" FROM templates WHERE 1=1 ");

        if let Some(id) = self.id {
            builder.push(" AND id = ");
            builder.push_bind(id);
        }

        if let Some(schema_id) = self.schema_id {
            builder.push(" AND schema_id = ");
            builder.push_bind(schema_id);
        }

        if let Some(channel_id) = self.channel_id {
            builder.push(" AND channel_id = ");
            builder.push_bind(channel_id);
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
    name: String,
    subject: Option<TemplateSnippet>,
    html: Option<TemplateSnippet>,
    text: Option<TemplateSnippet>,
    #[setters(skip)]
    schema_id: Id,
    #[setters(skip)]
    message_type_id: Id,
    #[setters(skip)]
    channel_id: Id,
}

impl Template {
    pub async fn save<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<Self, sqlx::Error> {
        sqlx::query_as(r#"
        INSERT INTO templates (id, name, subject, text, html, schema_id, message_type_id, channel_id) 
        VALUES($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING *"#
    )
    .bind(self.id)
    .bind(self.name)
    .bind(self.subject)
    .bind(self.text)
    .bind(self.html)
    .bind(self.schema_id)
    .bind(self.message_type_id)
    .bind(self.channel_id)
    .fetch_one(exec)
    .await
    .tap_err(log_query_error!())
    }

    pub async fn update<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<Self, sqlx::Error> {
        sqlx::query_as(
            r#"
            UPDATE templates 
            SET name=$1, subject=$2, text=$3, html=$4 
            WHERE id=$5 
            RETURNING *"#,
        )
        .bind(self.name)
        .bind(self.subject)
        .bind(self.text)
        .bind(self.html)
        .bind(self.id)
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
