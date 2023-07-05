use perroute_commons::types::{code::Code, id::Id};

#[derive(Debug, serde::Deserialize, Clone)]
pub struct CreateTemplateRequest {
    pub channel_id: Id,
    pub code: Code,
    pub description: String,
    pub html: Option<String>,
    pub text: Option<String>,
    pub subject: Option<String>,
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct UpdateTemplateRequest {
    pub description: String,
    pub html: Option<String>,
    pub text: Option<String>,
    pub subject: Option<String>,
}
