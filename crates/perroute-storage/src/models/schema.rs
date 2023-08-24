use super::{
    business_unit::{BusinessUnit, BusinessUnitQueryBuilder},
    message::{Message, MessageQuery},
    message_type::{MessageType, MessageTypeQueryBuilder},
    template::{Template, TemplatesQueryBuilder},
};
use crate::{
    query::{FetchableModel, ModelQueryBuilder, Projection},
    DatabaseModel,
};
use derive_builder::Builder;
use derive_getters::Getters;
use derive_setters::Setters;
use perroute_commons::types::{id::Id, json_schema::JsonSchema, vars::Vars, version::Version};
use perroute_connectors::types::dispatch_type::DispatchType;
use sqlx::{FromRow, PgExecutor};

impl DatabaseModel for Schema {}

#[derive(Debug, Default, Builder)]
pub struct SchemasQuery {
    #[builder(default)]
    id: Option<Id>,

    #[builder(default)]
    message_type_id: Option<Id>,

    #[builder(default)]
    business_unit_id: Option<Id>,

    #[builder(default)]
    version: Option<Version>,
}

impl SchemasQuery {
    pub fn with_id(id: Id) -> Self {
        Self {
            id: Some(id),
            ..Default::default()
        }
    }

    pub fn with_id_and_business_unit(id: Id, business_unit_id: Id) -> Self {
        Self {
            id: Some(id),
            business_unit_id: Some(business_unit_id),
            ..Default::default()
        }
    }

    pub fn with_message_type_id(message_type_id: Id) -> Self {
        Self {
            message_type_id: Some(message_type_id),
            ..Default::default()
        }
    }
}

impl ModelQueryBuilder<Schema> for SchemasQuery {
    fn build(&self, projection: Projection) -> sqlx::QueryBuilder<'_, sqlx::Postgres> {
        let mut builder = projection.query_builder();

        builder.push(
            r#" 
                FROM schemas s                 
                WHERE 1=1 "#,
        );

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
        dispatch_type: &DispatchType,
    ) -> Result<Option<Template>, sqlx::Error> {
        todo!()
    }

    pub async fn templates<'e, E: PgExecutor<'e>>(
        &self,
        exec: E,
    ) -> Result<Vec<Template>, sqlx::Error> {
        Template::query(
            exec,
            TemplatesQueryBuilder::default()
                .schema_id(Some(self.id))
                .build()
                .unwrap(),
        )
        .await
    }

    pub async fn message_type<'e, E: PgExecutor<'e>>(
        &self,
        exec: E,
    ) -> Result<MessageType, sqlx::Error> {
        MessageType::find_one(
            exec,
            MessageTypeQueryBuilder::default()
                .id(Some(self.message_type_id))
                .build()
                .unwrap(),
        )
        .await
    }

    pub async fn business_unit<'e, E: PgExecutor<'e>>(
        &self,
        exec: E,
    ) -> Result<BusinessUnit, sqlx::Error> {
        BusinessUnit::find_one(
            exec,
            BusinessUnitQueryBuilder::default()
                .id(Some(self.business_unit_id))
                .build()
                .unwrap(),
        )
        .await
    }

    pub async fn exists_messages<'e, E: PgExecutor<'e>>(
        &self,
        exec: E,
    ) -> Result<bool, sqlx::Error> {
        Message::exists(exec, MessageQuery::with_id(self.id)).await
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

    pub async fn next_version(
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
        .map(|r| r.map(|v| v.increment()).unwrap_or_default())
    }
}
