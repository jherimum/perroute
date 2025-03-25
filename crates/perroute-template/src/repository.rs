#[cfg(feature = "repo_aws_s3")]
pub mod aws_s3;

use std::fmt::Debug;

use perroute_commons::types::{dispatch_type::DispatchType, id::Id};
use crate::template::{NotRenderedTemplateState, Template};

#[derive(Debug, thiserror::Error)]
pub enum TemplateRepositoryError {
    #[cfg(feature = "repo_aws_s3")]
    #[error("")]
    AwsS3TemplateRepositoryError(#[from] crate::repository::aws_s3::Error),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct TemplateId<'i> {
    template_assignment_id: &'i Id,
    dispatch_type: &'i DispatchType,
    template_id: &'i Id,
}

impl<'i> TemplateId<'i> {
    pub fn template_assignment_id(&self) -> &Id {
        &self.template_assignment_id
    }

    pub fn dispatch_type(&self) -> &DispatchType {
        &self.dispatch_type
    }

    pub fn template_id(&self) -> &Id {
        &self.template_id
    }

    pub fn new(
        template_assignment_id: &'i Id,
        dispatch_type: &'i DispatchType,
        template_id: &'i Id,
    ) -> Self {
        Self {
            template_assignment_id,
            dispatch_type,
            template_id,
        }
    }
}

#[cfg(feature = "test-mocks")]
use mockall::automock;

#[cfg_attr(feature = "test-mocks", automock)]
#[async_trait::async_trait]
pub trait TemplateRepository {}

#[cfg_attr(feature = "test-mocks", automock)]
#[async_trait::async_trait]
pub trait TemplateLookup {
    async fn get<'a>(
        &self,
        id: &'a TemplateId<'a>,
    ) -> Result<
        Option<Template<NotRenderedTemplateState>>,
        TemplateRepositoryError,
    >;
}
