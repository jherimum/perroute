use derive_builder::Builder;
use derive_getters::Getters;
use derive_setters::Setters;
use perroute_commons::types::{id::Id, template::TemplateSnippet};
use sqlx::{FromRow, PgExecutor};

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
    pub async fn query<'e, E: PgExecutor<'e>>(exec: E) -> Result<Vec<Self>, sqlx::Error> {
        todo!("Implement Template::query")
    }

    pub async fn save<'e, E: PgExecutor<'e>>(&self, exec: E) -> Result<Self, sqlx::Error> {
        todo!("Implement Template::save")
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, exec: E) -> Result<Self, sqlx::Error> {
        todo!("Implement Template::update")
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(&self, exec: E) -> Result<bool, sqlx::Error> {
        todo!("Implement Template::delete")
    }

    pub async fn find_by_id<'e, E: PgExecutor<'e>>(
        exec: E,
        id: Id,
    ) -> Result<Option<Self>, sqlx::Error> {
        todo!("Implement Template::delete")
    }
}
