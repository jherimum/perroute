use crate::api::response::CollectionResourceModel;
use crate::api::response::Links;
use crate::api::response::ResourceBuilder;
use crate::api::response::SingleResourceModel;
use perroute_storage::models::template::Template;
use serde::Serialize;
use validator::Validate;

#[derive(Debug, serde::Deserialize, Clone, Validate, Default)]
#[serde(default)]
pub struct CreateTemplateRequest {
    #[validate(custom = "perroute_commons::types::name::validate")]
    pub name: String,

    #[validate(custom = "perroute_commons::types::id::Id::validate")]
    pub business_unit_id: String,

    #[validate(custom = "perroute_commons::types::id::Id::validate")]
    pub message_type_id: String,

    pub subject: Option<String>,
    pub html: Option<String>,
    pub text: Option<String>,

    #[validate(custom = "perroute_connectors::types::DispatchType::validate")]
    pub dispatch_type: String,
}

#[derive(Debug, serde::Deserialize, Clone, Validate, Default)]
#[serde(default)]
pub struct UpdateTemplateRequest {
    #[validate(custom = "perroute_commons::types::name::validate")]
    pub name: String,

    pub subject: Option<String>,
    pub html: Option<String>,
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct TemplateResource {
    pub id: String,
    pub name: String,
    pub subject: Option<String>,
    pub html: Option<String>,
    pub text: Option<String>,
    pub dispatch_type: String,
}

impl From<Template> for TemplateResource {
    fn from(template: Template) -> Self {
        Self {
            id: template.id().into(),
            name: template.name().into(),
            subject: template.subject().clone(),
            html: template.html().clone().map(Into::into),
            text: template.text().clone().map(Into::into),
            dispatch_type: template.dispatch_type().into(),
        }
    }
}

impl ResourceBuilder<SingleResourceModel<TemplateResource>> for Template {
    fn build(&self, req: &actix_web::HttpRequest) -> SingleResourceModel<TemplateResource> {
        SingleResourceModel {
            data: Some(TemplateResource::from(self.clone())),
            links: Links::default().as_url_map(req),
        }
    }
}

impl ResourceBuilder<CollectionResourceModel<TemplateResource>> for Vec<Template> {
    fn build(&self, req: &actix_web::HttpRequest) -> CollectionResourceModel<TemplateResource> {
        CollectionResourceModel {
            data: self.iter().map(|c| c.build(req)).collect(),
            links: Links::default().as_url_map(req),
        }
    }
}
