use super::{
    business_unit::{BusinessUnit, BusinessUnitQueryBuilder},
    message_type::{MessageType, MessageTypeQueryBuilder},
};
use crate::{
    log_query_error,
    query::{FetchableModel, ModelQueryBuilder, Projection},
    DatabaseModel, Result,
};
use chrono::NaiveDateTime;
use derive_builder::Builder;
use derive_getters::Getters;
use derive_setters::Setters;
use perroute_commons::types::{id::Id, priority::Priority, template::TemplateSnippet, vars::Vars};
use perroute_connectors::types::dispatch_type::DispatchType;
use sqlx::{FromRow, PgExecutor, QueryBuilder};
use std::ops::Deref;
use tap::TapFallible;

#[derive(Debug, Builder, Default)]
#[builder(default)]
pub struct TemplatesQuery {
    id: Option<Id>,
    message_type_id: Option<Id>,
    business_unit_id: Option<Id>,
    dispatch_type: Option<DispatchType>,
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

        if let Some(business_unit_id) = self.business_unit_id {
            builder.push(" AND business_unit_id = ");
            builder.push_bind(business_unit_id);
        }

        if let Some(message_type_id) = self.message_type_id {
            builder.push(" AND message_type_id = ");
            builder.push_bind(message_type_id);
        }

        if let Some(dispath_type) = self.dispatch_type {
            builder.push(" AND dispatch_type = ");
            builder.push_bind(dispath_type);
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
    active: bool,

    #[setters(skip)]
    dispatch_type: DispatchType,
}

impl Template {
    pub async fn find_active_template<'e, E: PgExecutor<'e>>(
        exec: E,
        dispatch_type: &DispatchType,
        instant: &NaiveDateTime,
    ) -> Result<Option<Template>> {
        Ok(sqlx::query_as(
            r#"
                    SELECT * 
                    FROM templates 
                    WHERE 
                        dispatch_type = $1
                        AND start_at <= $2 
                        AND (end_at is null OR end_at >= $2 ) 
                        AND active = true               
                        ORDER BY priority desc
                        LIMIT 1"#,
        )
        .bind(dispatch_type)
        .bind(instant)
        .fetch_optional(exec)
        .await?)
    }

    pub async fn save<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<Self> {
        Ok(sqlx::query_as(
            r#"
        INSERT INTO templates (
            id, 
            dispatch_type, 
            subject, 
            text, 
            html, 
            active, 
            name) 
        VALUES($1, $2, $3, $4, $5, $6, $7)
        RETURNING *"#,
        )
        .bind(self.id)
        .bind(self.dispatch_type)
        .bind(self.subject)
        .bind(self.text)
        .bind(self.html)
        .bind(self.active)
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
                active=$5, 
                name=$6,
            WHERE id=$1 
            RETURNING *"#,
        )
        .bind(self.id)
        .bind(self.subject)
        .bind(self.text)
        .bind(self.html)
        .bind(self.active)
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
