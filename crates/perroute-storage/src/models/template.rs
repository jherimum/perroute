use derive_builder::Builder;
use derive_getters::Getters;
use derive_setters::Setters;
use perroute_commons::types::{code::Code, id::Id, template::TemplateSnippet};
use sqlx::{FromRow, PgExecutor};

#[derive(Debug, FromRow, PartialEq, Eq, Clone, Getters, Setters, Builder)]
#[builder(setter(into))]
#[setters(prefix = "set_")]
#[setters(into)]
pub struct Template {
    #[setters(skip)]
    id: Id,

    #[setters(skip)]
    code: Code,

    description: String,
    html: TemplateSnippet,
    text: TemplateSnippet,
    subject: TemplateSnippet,

    #[setters(skip)]
    schema_id: Id,
}

impl Template {
    pub async fn query<'e, E: PgExecutor<'e>>(exec: E) -> Result<Vec<Self>, sqlx::Error> {
        todo!("Implement Template::query")
    }

    pub async fn save<'e, E: PgExecutor<'e>>(&self, exec: E) -> Result<Self, sqlx::Error> {
        todo!("Implement Template::save")
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, exec: E) -> Result<Self, sqlx::Error> {
        todo!("Implement Template::update")
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, exec: E) -> Result<Self, sqlx::Error> {
        todo!("Implement Template::delete")
    }
}
