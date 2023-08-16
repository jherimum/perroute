use crate::api::response::CollectionResourceModel;
use crate::api::response::Links;
use crate::api::response::ResourceBuilder;
use crate::api::response::SingleResourceModel;
use perroute_commons::types::id::Id;
use perroute_connectors::types::DispatchType;
use perroute_storage::models::template::Template;
use serde::Serialize;

#[derive(Debug, serde::Deserialize, Clone)]
pub struct CreateTemplateRequest {
    pub business_unit_id: Id,
    pub message_type_id: Id,
    pub subject: Option<String>,
    pub html: Option<String>,
    pub text: Option<String>,
    pub dispatch_type: DispatchType,
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct UpdateTemplateRequest {
    pub subject: Option<String>,
    pub html: Option<String>,
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct TemplateResource {
    pub id: Id,
    pub subject: Option<String>,
    pub html: Option<String>,
    pub text: Option<String>,
    pub dispatch_type: DispatchType,
}

impl From<Template> for TemplateResource {
    fn from(template: Template) -> Self {
        Self {
            id: *template.id(),
            subject: template.subject().clone().map(Into::into),
            html: template.html().clone().map(Into::into),
            text: template.text().clone().map(Into::into),
            dispatch_type: *template.dispatch_type(),
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
