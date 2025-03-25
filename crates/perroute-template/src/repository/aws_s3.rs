use aws_config::SdkConfig;
use aws_sdk_s3::{
    config::http::HttpResponse, error::SdkError,
    operation::get_object::GetObjectError,
};
use perroute_commons::types::dispatch_type::DispatchType;
use crate::template::{NotRenderedTemplateState, Template};
use super::{TemplateId, TemplateLookup, TemplateRepositoryError};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("")]
    SdkError(#[from] SdkError<GetObjectError, HttpResponse>),

    #[error("")]
    Serde(#[from] serde_json::Error),

    #[error("")]
    InvalidTemplate,
}

#[derive(Clone)]
pub struct AwsS3TemplateRepository {
    client: aws_sdk_s3::Client,
    bucket: String,
}

impl AwsS3TemplateRepository {
    pub fn new(cfg: &SdkConfig, bucket: &str) -> Self {
        Self {
            client: aws_sdk_s3::Client::new(cfg),
            bucket: bucket.to_string(),
        }
    }
}

#[async_trait::async_trait]
impl TemplateLookup for AwsS3TemplateRepository {
    async fn get<'a>(
        &self,
        id: &'a TemplateId<'a>,
    ) -> Result<
        Option<Template<NotRenderedTemplateState>>,
        TemplateRepositoryError,
    > {
        let response = self
            .client
            .get_object()
            .bucket(&self.bucket)
            .key(to_key(&id))
            .send()
            .await
            .map_err(Error::from)?;

        let bytes = response.body().bytes().unwrap();
        let template: Template<NotRenderedTemplateState> =
            serde_json::from_reader(bytes).map_err(Error::from)?;

        let expected = match (id.dispatch_type(), &template) {
            (DispatchType::Email, Template::Email(_)) => true,
            (DispatchType::Sms, Template::Sms(_)) => true,
            (DispatchType::Push, Template::Push(_)) => true,
            _ => false,
        };

        match expected {
            true => Ok(Some(template)),
            false => Err(Error::InvalidTemplate.into()),
        }
    }
}

fn to_key(id: &TemplateId) -> String {
    todo!()
}
