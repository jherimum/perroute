use perroute_commons::types::id::Id;

#[derive(Debug, serde::Deserialize, Clone)]
pub struct CreateTemplateRequest {
    pub schema_id: Id,
    pub name: String,
    pub html: Option<String>,
    pub text: Option<String>,
    pub subject: Option<String>,
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct UpdateTemplateRequest {
    pub name: String,
    pub html: Option<String>,
    pub text: Option<String>,
    pub subject: Option<String>,
}
