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

impl From<TemplateSnippet> for String {
    fn from(value: TemplateSnippet) -> Self {
        value.0
    }
}
