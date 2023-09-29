use crate::api::response::CollectionResourceModel;
use crate::api::response::Links;
use crate::api::response::ResourceBuilder;
use crate::api::response::SingleResourceModel;
use crate::links::Linkrelation;
use crate::links::ResourceLink;
use anyhow::Context;
use anyhow::Result;
use perroute_commons::types::template::TemplateSnippet;
use perroute_commons::types::template::TemplateValidator;
use perroute_storage::models::template::Template;
use serde::Serialize;
use validator::Validate;

#[derive(Debug, serde::Deserialize, Clone, Validate, Default)]
#[serde(default)]
pub struct CreateTemplateRequest {
    #[validate(required)]
    #[validate(custom = "perroute_commons::types::name::validate")]
    name: Option<String>,

    #[validate(custom = "perroute_commons::types::template::handlebars::Handlebars::validate")]
    subject: Option<String>,

    #[validate(custom = "perroute_commons::types::template::handlebars::Handlebars::validate")]
    html: Option<String>,

    #[validate(custom = "perroute_commons::types::template::handlebars::Handlebars::validate")]
    text: Option<String>,
}

impl CreateTemplateRequest {
    pub fn name(&self) -> Result<String> {
        self.name.clone().context("missing name")
    }

    pub fn subject(&self) -> Result<Option<TemplateSnippet>> {
        Ok(self.subject.clone().map(Into::into))
    }

    pub fn html(&self) -> Result<Option<TemplateSnippet>> {
        Ok(self.html.clone().map(Into::into))
    }

    pub fn text(&self) -> Result<Option<TemplateSnippet>> {
        Ok(self.text.clone().map(Into::into))
    }
}

#[derive(Debug, serde::Deserialize, Clone, Validate, Default)]
#[serde(default)]
pub struct UpdateTemplateRequest {
    #[validate(custom = "perroute_commons::types::name::validate")]
    name: Option<String>,

    #[serde(
        default,                                    // <- important for deserialization
        with = "::serde_with::rust::double_option",
    )]
    subject: Option<Option<String>>,

    #[serde(
        default,                                    // <- important for deserialization
        with = "::serde_with::rust::double_option",
    )]
    html: Option<Option<String>>,

    #[serde(
        default,                                    // <- important for deserialization
        with = "::serde_with::rust::double_option",
    )]
    text: Option<Option<String>>,
}

impl UpdateTemplateRequest {
    pub fn name(&self) -> Result<Option<String>> {
        Ok(self.name.clone())
    }

    pub fn subject(&self) -> Result<Option<Option<TemplateSnippet>>> {
        Ok(self.subject.clone().map(|s| s.map(Into::into)))
    }

    pub fn html(&self) -> Result<Option<Option<TemplateSnippet>>> {
        Ok(self.html.clone().map(|s| s.map(Into::into)))
    }

    pub fn text(&self) -> Result<Option<Option<TemplateSnippet>>> {
        Ok(self.text.clone().map(|s| s.map(Into::into)))
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct TemplateResource {
    pub id: String,
    pub name: String,
    pub subject: Option<String>,
    pub html: Option<String>,
    pub text: Option<String>,
}

impl From<&Template> for TemplateResource {
    fn from(template: &Template) -> Self {
        Self {
            id: template.id().into(),
            name: template.name().into(),
            subject: template.subject().clone().map(Into::into),
            html: template.html().clone().map(Into::into),
            text: template.text().clone().map(Into::into),
        }
    }
}

impl ResourceBuilder<SingleResourceModel<TemplateResource>> for Template {
    fn build(&self, req: &actix_web::HttpRequest) -> SingleResourceModel<TemplateResource> {
        SingleResourceModel {
            data: Some(TemplateResource::from(self)),
            links: Links::default()
                .add(Linkrelation::Self_, ResourceLink::Template(*self.id()))
                .add(Linkrelation::Templates, ResourceLink::Templates)
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
