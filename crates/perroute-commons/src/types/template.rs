use serde::Serialize;
use sqlx::Type;

#[derive(Debug, Clone, PartialEq, Eq, Type, Serialize)]
#[sqlx(transparent)]
pub struct TemplateSnippet(String);

impl From<String> for TemplateSnippet {
    fn from(value: String) -> Self {
        TemplateSnippet(value)
    }
}
