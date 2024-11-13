use serde::de::DeserializeOwned;

pub enum TemplateError {}

pub trait TemplateRender {
    fn render<CTX: DeserializeOwned>(
        &self,
        template: &str,
        context: CTX,
    ) -> Result<String, TemplateError>
    where
        Self: Sized;
}
