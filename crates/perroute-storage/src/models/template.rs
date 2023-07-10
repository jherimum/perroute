use derive_builder::Builder;
use derive_getters::Getters;
use derive_setters::Setters;
use perroute_commons::types::{id::Id, template::TemplateSnippet};
use sqlx::{FromRow, PgExecutor, QueryBuilder};

use crate::{
    query::{ModelQuery, Projection},
    DatabaseModel,
};

impl DatabaseModel for Template {}

#[derive(Debug, Builder)]
pub struct TemplatesQuery {
    id: Option<Id>,
    schema_id: Option<Id>,
    message_type_id: Option<Id>,
    channel_id: Option<Id>,
}

impl ModelQuery<Template> for TemplatesQuery {
    fn query_builder(&self, projection: Projection) -> QueryBuilder<'_, sqlx::Postgres> {
        let mut builder = projection.query_builder();

        builder.push(" FROM templates ");

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
    pub async fn save<'e, E: PgExecutor<'e>>(&self, exec: E) -> Result<Self, sqlx::Error> {
        todo!("Implement Template::save")
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, exec: E) -> Result<Self, sqlx::Error> {
        todo!("Implement Template::update")
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, exec: E) -> Result<bool, sqlx::Error> {
        todo!("Implement Template::delete")
    }
}
