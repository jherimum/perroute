use std::collections::HashMap;

use crate::api::response::CollectionResourceModel;
use crate::api::response::Links;
use crate::api::response::ResourceBuilder;
use crate::api::response::SingleResourceModel;
use crate::links::Linkrelation;
use crate::links::ResourceLink;
use perroute_storage::models::template::Template;
use serde::Serialize;
use validator::Validate;

#[derive(Debug, serde::Deserialize, Clone, Validate, Default)]
#[serde(default)]
pub struct CreateTemplateRequest {
    #[validate(custom = "perroute_commons::types::name::validate")]
    pub name: String,
    pub subject: Option<String>,
    pub html: Option<String>,
    pub text: Option<String>,
    pub vars: HashMap<String, String>,

    #[validate(custom = "perroute_connectors::types::dispatch_type::DispatchType::validate")]
    pub dispatch_type: String,

    #[validate(custom = "perroute_commons::types::id::Id::validate")]
    pub schema_id: String,
}

#[derive(Debug, serde::Deserialize, Clone, Validate, Default)]
#[serde(default)]
pub struct UpdateTemplateRequest {
    #[validate(custom = "perroute_commons::types::name::validate")]
    pub name: Option<String>,
    pub subject: Option<Option<String>>,
    pub html: Option<Option<String>>,
    pub text: Option<Option<String>>,
    pub vars: Option<HashMap<String, String>>,
    pub active: Option<bool>,
}

#[derive(Debug, Serialize, Clone)]
pub struct TemplateResource {
    pub id: String,
    pub name: String,
    pub subject: Option<String>,
    pub html: Option<String>,
    pub text: Option<String>,
    pub dispatch_type: String,
    pub active: bool,
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
            active: *template.active(),
        }
    }
}

impl ResourceBuilder<SingleResourceModel<TemplateResource>> for Template {
    fn build(&self, req: &actix_web::HttpRequest) -> SingleResourceModel<TemplateResource> {
        SingleResourceModel {
            data: Some(TemplateResource::from(self.clone())),
            links: Links::default()
                .add(
                    Linkrelation::Schema,
                    ResourceLink::Schema(*self.message_type_id(), *self.schema_id()),
                )
                .as_url_map(req),
        }
    }
}

impl ResourceBuilder<CollectionResourceModel<TemplateResource>> for Vec<Template> {
    fn build(&self, req: &actix_web::HttpRequest) -> CollectionResourceModel<TemplateResource> {
        CollectionResourceModel {
            data: self.iter().map(|c| c.build(req)).collect(),
            links: Links::default()
                .add(Linkrelation::Self_, ResourceLink::Templates)
                .as_url_map(req),
        }
    }
}
