use super::{
    business_unit::{BusinessUnit, BusinessUnitQueryBuilder},
    message_type::{MessageType, MessageTypeQueryBuilder},
    schema::{Schema, SchemasQueryBuilder},
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
    schema_id: Option<Id>,
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

    pub fn with_schema_id(schema_id: Id) -> Self {
        Self {
            schema_id: Some(schema_id),
            ..Default::default()
        }
    }

    pub fn with_schema_id_and_dispatch_type(schema_id: Id, dispatch_type: DispatchType) -> Self {
        Self {
            schema_id: Some(schema_id),
            dispatch_type: Some(dispatch_type),
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

        if let Some(schema_id) = self.schema_id {
            builder.push(" AND schema_id = ");
            builder.push_bind(schema_id);
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
    vars: Vars,
    active: bool,
    start_at: NaiveDateTime,
    end_at: Option<NaiveDateTime>,
    priority: Priority,

    #[setters(skip)]
    dispatch_type: DispatchType,

    #[setters(skip)]
    schema_id: Id,

    #[setters(skip)]
    message_type_id: Id,

    #[setters(skip)]
    business_unit_id: Id,
}

impl Template {
    pub async fn message_type<'e, E: PgExecutor<'e>>(&self, exec: E) -> Result<MessageType> {
        MessageType::find_one(
            exec,
            MessageTypeQueryBuilder::default()
                .id(Some(self.message_type_id))
                .build()
                .unwrap(),
        )
        .await
    }

    pub async fn schema<'e, E: PgExecutor<'e>>(&self, exec: E) -> Result<Schema> {
        Schema::find_one(
            exec,
            SchemasQueryBuilder::default()
                .id(Some(self.schema_id))
                .build()
                .unwrap(),
        )
        .await
    }

    pub async fn business_unit<'e, E: PgExecutor<'e>>(&self, exec: E) -> Result<BusinessUnit> {
        BusinessUnit::find_one(
            exec,
            BusinessUnitQueryBuilder::default()
                .id(Some(self.business_unit_id))
                .build()
                .unwrap(),
        )
        .await
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
            vars, 
            active, 
            schema_id, 
            message_type_id, 
            business_unit_id, 
            name,
            start_at,
            end_at,
            priority) 
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
        .bind(self.business_unit_id)
        .bind(self.name)
        .bind(self.start_at)
        .bind(self.end_at)
        .bind(self.priority)
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
                vars=$5, 
                active=$6, 
                name=$7,
                start_at=$8,
                end_at=$9,
                priority=$10
            WHERE id=$1 
            RETURNING *"#,
        )
        .bind(self.id)
        .bind(self.subject)
        .bind(self.text)
        .bind(self.html)
        .bind(self.vars)
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

    pub async fn inactivate_all<'e, E: PgExecutor<'e>>(
        exec: E,
        schema_id: Id,
        dispatch_type: DispatchType,
    ) -> Result<u64> {
        Ok(sqlx::query(
            "UPDATE templates SET active = false WHERE schema_id = $1 AND dispatch_type = $2",
        )
        .bind(schema_id)
        .bind(dispatch_type)
        .execute(exec)
        .await
        .tap_err(log_query_error!())
        .map(|result| result.rows_affected())?)
    }
}
