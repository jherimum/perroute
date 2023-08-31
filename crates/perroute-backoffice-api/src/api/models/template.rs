use crate::api::response::CollectionResourceModel;
use crate::api::response::Links;
use crate::api::response::ResourceBuilder;
use crate::api::response::SingleResourceModel;
use crate::links::Linkrelation;
use crate::links::ResourceLink;
use anyhow::Context;
use anyhow::Result;
use perroute_commons::types::id::Id;
use perroute_commons::types::template::TemplateSnippet;
use perroute_commons::types::template::TemplateValidator;
use perroute_connectors::types::dispatch_type::DispatchType;
use perroute_storage::models::template::Template;
use serde::Serialize;
use std::collections::HashMap;
use validator::Validate;

#[derive(Debug, serde::Deserialize, Clone, Validate, Default)]
#[serde(default)]
pub struct CreateTemplateRequest {
    #[validate(required)]
    #[validate(custom = "Id::validate")]
    schema_id: Option<String>,

    #[validate(required)]
    #[validate(custom = "DispatchType::validate")]
    dispatch_type: Option<String>,

    #[validate(required)]
    #[validate(custom = "perroute_commons::types::name::validate")]
    name: Option<String>,

    #[validate(custom = "perroute_commons::types::template::handlebars::Handlebars::validate")]
    subject: Option<String>,

    #[validate(custom = "perroute_commons::types::template::handlebars::Handlebars::validate")]
    html: Option<String>,

    #[validate(custom = "perroute_commons::types::template::handlebars::Handlebars::validate")]
    text: Option<String>,

    vars: Option<HashMap<String, String>>,
}

impl CreateTemplateRequest {
    pub fn schema_id(&self) -> Result<Id> {
        Ok(self
            .schema_id
            .clone()
            .context("missing schema id")?
            .try_into()
            .context("invalid schema id")?)
    }

    pub fn dispatch_type(&self) -> Result<DispatchType> {
        Ok(self
            .dispatch_type
            .clone()
            .context("missing dispatch type")?
            .try_into()
            .context("invalid dispatch type")?)
    }

    pub fn name(&self) -> Result<String> {
        Ok(self.name.clone().context("missing name")?)
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

    vars: Option<HashMap<String, String>>,
    active: Option<bool>,
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

    pub fn active(&self) -> Result<Option<bool>> {
        Ok(self.active.clone())
    }
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

impl From<&Template> for TemplateResource {
    fn from(template: &Template) -> Self {
        Self {
            id: template.id().into(),
            name: template.name().into(),
            subject: template.subject().clone().map(Into::into),
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
            data: Some(TemplateResource::from(self)),
            links: Links::default()
                .add(
                    Linkrelation::Schema,
                    ResourceLink::Schema(*self.schema_id()),
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
