use crate::api::response::CollectionResourceModel;
use crate::api::response::Links;
use crate::api::response::ResourceBuilder;
use crate::api::response::SingleResourceModel;
use crate::api::types::RestDateTime;
use crate::links::Linkrelation;
use crate::links::ResourceLink;
use anyhow::Context;
use anyhow::Result;
use perroute_commons::types::id::Id;
use perroute_commons::types::priority::Priority;
use perroute_commons::types::template::TemplateSnippet;
use perroute_commons::types::template::TemplateValidator;
use perroute_commons::types::vars::Vars;
use perroute_connectors::types::dispatch_type::DispatchType;
use perroute_storage::models::template::Template;
use serde::Serialize;
use sqlx::types::chrono::NaiveDateTime;
use std::borrow::Cow;
use std::collections::HashMap;
use std::str::FromStr;
use validator::Validate;
use validator::ValidationError;

#[derive(Debug, serde::Deserialize, Clone, Validate, Default)]
#[serde(default)]
#[validate(schema(function = "validate_create_date_range"))]
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

    #[validate(required)]
    #[validate(custom = "RestDateTime::validate")]
    start_at: Option<String>,

    #[validate(custom = "RestDateTime::validate")]
    end_at: Option<String>,

    #[validate(required)]
    #[validate(custom = "Priority::validate")]
    priority: Option<i32>,
}

fn validate_create_date_range(req: &CreateTemplateRequest) -> Result<(), ValidationError> {
    if let (Some(start_at), Some(end_at)) = (&req.start_at, &req.end_at) {
        let start_at = RestDateTime::from_str(start_at)
            .context("Invalid start_at")
            .unwrap();
        let end_at = RestDateTime::from_str(end_at)
            .context("Invalid end_at")
            .unwrap();

        if start_at > end_at {
            return Err(ValidationError {
                code: Cow::Borrowed("dates"),
                message: Some(Cow::Borrowed("Invalid date range")),
                params: Default::default(),
            });
        }
    }

    Ok(())
}

impl CreateTemplateRequest {
    pub fn schema_id(&self) -> Result<Id> {
        self.schema_id
            .clone()
            .context("missing schema id")?
            .try_into()
            .context("invalid schema id")
    }

    pub fn dispatch_type(&self) -> Result<DispatchType> {
        self.dispatch_type
            .clone()
            .context("missing dispatch type")?
            .try_into()
            .context("invalid dispatch type")
    }

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

    pub fn vars(&self) -> Result<Vars> {
        Ok(self.vars.clone().unwrap_or_default().into())
    }

    pub fn priority(&self) -> Result<Priority> {
        self.priority
            .context("Missing priority")?
            .try_into()
            .context("Invalid priority")
    }

    pub fn start_at(&self) -> Result<NaiveDateTime> {
        Ok(self
            .start_at
            .clone()
            .context("Missing start_at")?
            .parse::<RestDateTime>()
            .context("Invalid start_at")?
            .into())
    }

    pub fn end_at(&self) -> Result<Option<NaiveDateTime>> {
        Ok(self
            .end_at
            .clone()
            .map(|s| s.parse::<RestDateTime>().context("Invalid end_at"))
            .transpose()?
            .map(Into::into))
    }
}

#[derive(Debug, serde::Deserialize, Clone, Validate, Default)]
#[serde(default)]
#[validate(schema(function = "validate_update_date_range"))]
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

    #[validate(required)]
    #[validate(custom = "RestDateTime::validate")]
    start_at: Option<String>,

    #[serde(
        default,                                    // <- important for deserialization
        with = "::serde_with::rust::double_option",
    )]
    #[validate(custom = "RestDateTime::validate")]
    end_at: Option<Option<String>>,

    #[validate(required)]
    #[validate(custom = "Priority::validate")]
    priority: Option<i32>,
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
        Ok(self.active)
    }

    pub fn priority(&self) -> Result<Option<Priority>> {
        Ok(self.priority.map(|p| p.try_into()).transpose()?)
    }

    pub fn start_at(&self) -> Result<Option<NaiveDateTime>> {
        Ok(self
            .start_at
            .clone()
            .map(|s| RestDateTime::from_str(&s).context("Invalid start_at"))
            .transpose()?
            .map(Into::into))
    }

    pub fn end_at(&self) -> Result<Option<Option<NaiveDateTime>>> {
        Ok(self
            .text
            .clone()
            .map(|s| {
                s.map(|s| RestDateTime::from_str(&s).context("Invalid end_at"))
                    .transpose()
            })
            .transpose()?
            .map(|s| s.map(Into::into)))
    }
}

fn validate_update_date_range(req: &UpdateTemplateRequest) -> Result<(), ValidationError> {
    if let (Some(start_at), Some(Some(end_at))) = (&req.start_at, &req.end_at) {
        let start_at = RestDateTime::from_str(start_at)
            .context("Invalid start_at")
            .unwrap();
        let end_at = RestDateTime::from_str(end_at)
            .context("Invalid end_at")
            .unwrap();

        if start_at > end_at {
            return Err(ValidationError {
                code: Cow::Borrowed("dates"),
                message: Some(Cow::Borrowed("Invalid date range")),
                params: Default::default(),
            });
        }
    }

    Ok(())
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
                .add(Linkrelation::Self_, ResourceLink::Template(*self.id()))
                .add(Linkrelation::Templates, ResourceLink::Templates)
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
