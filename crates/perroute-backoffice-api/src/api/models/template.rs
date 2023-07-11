use crate::api::response::Links;
use crate::api::response::{ResourceBuilder, ResourceModel};
use perroute_commons::types::{id::Id, template::TemplateSnippet};
use perroute_storage::models::template::Template;
use serde::Serialize;
use std::ops::Deref;

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

#[derive(Debug, Serialize, Clone)]
pub struct TemplateResource {
    pub id: Id,
    pub name: String,
    pub html: Option<String>,
    pub text: Option<String>,
    pub subject: Option<String>,
}

impl From<Template> for TemplateResource {
    fn from(template: Template) -> Self {
        TemplateResource {
            id: *template.id(),
            name: template.name().to_owned(),
            html: template.html().clone().map(Into::into),
            text: template.text().clone().map(Into::into),
            subject: template.subject().clone().map(Into::into),
        }
    }
}

impl ResourceBuilder<ResourceModel<TemplateResource>> for Template {
    fn build(&self, req: &actix_web::HttpRequest) -> ResourceModel<TemplateResource> {
        ResourceModel {
            data: Some(TemplateResource::from(self.clone())),
            links: Links::default().as_url_map(req),
        }
    }
}

impl ResourceBuilder<ResourceModel<Vec<ResourceModel<TemplateResource>>>> for Vec<Template> {
    fn build(
        &self,
        req: &actix_web::HttpRequest,
    ) -> ResourceModel<Vec<ResourceModel<TemplateResource>>> {
        ResourceModel {
            data: Some(self.iter().map(|c| c.build(req)).collect()),
            links: Links::default().as_url_map(req),
        }
    }
}
