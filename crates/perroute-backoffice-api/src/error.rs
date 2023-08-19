use actix_web::ResponseError;
use perroute_commons::{
    rest::RestError,
    types::{id::Id, json_schema::InvalidSchemaError},
};
use perroute_cqrs::{command_bus::error::CommandBusError, query_bus::error::QueryBusError};
use std::collections::HashMap;
use thiserror::Error;
use validator::{ValidationError, ValidationErrors, ValidationErrorsKind};

#[derive(Debug, Error)]
pub enum ApiError {
    #[error(transparent)]
    ValidationError(#[from] actix_web_validator::Error),

    #[error(transparent)]
    InvalidSchema(#[from] InvalidSchemaError),

    #[error("Business unit {0} not found")]
    BusinessUnitNotFound(Id),

    #[error("ApiKey {0} not found")]
    ApiKeyNotFound(Id),

    #[error("Message type {0} not found")]
    MessageTypeNotFound(Id),

    #[error("Schema {0} not found")]
    SchemaNotFound(Id),

    #[error("Template {0} not found")]
    TemplateNotFound(Id),

    #[error(transparent)]
    CommandBus(#[from] CommandBusError),

    #[error(transparent)]
    QueryBus(#[from] QueryBusError),

    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
}

impl From<&ApiError> for RestError {
    fn from(value: &ApiError) -> Self {
        match value {
            ApiError::BusinessUnitNotFound(_)
            | ApiError::MessageTypeNotFound(_)
            | ApiError::SchemaNotFound(_)
            | ApiError::TemplateNotFound(_) => Self::NotFound(value.to_string()),
            ApiError::CommandBus(CommandBusError::ExpectedError(message)) => {
                Self::UnprocessableEntity(message.to_string())
            }
            ApiError::ValidationError(error) => match error {
                actix_web_validator::Error::Validate(e) => {
                    RestError::BadRequest(validation_errors_to_hashmap(e))
                }
                actix_web_validator::Error::Deserialize(e) => {
                    tracing::error!("xxxxxxxxxx{e:?}");
                    todo!()
                }
                actix_web_validator::Error::JsonPayloadError(e) => {
                    tracing::error!("xxxxxxxxxx{e:?}");
                    todo!()
                }
                actix_web_validator::Error::UrlEncodedError(e) => {
                    tracing::error!("xxxxxxxxxx{e:?}");
                    todo!()
                }
                actix_web_validator::Error::QsError(e) => {
                    tracing::error!("xxxxxxxxxx{e:?}");
                    todo!()
                }
            },
            _ => Self::InternalServer,
        }
    }
}

fn validation_errors_to_hashmap(errors: &ValidationErrors) -> Option<HashMap<String, String>> {
    if errors.is_empty() {
        return None;
    }

    let mut map = HashMap::new();
    for (key, kind) in errors.errors() {
        map.extend(build_kind(key.to_string(), kind));
    }

    Some(map)
}

fn build_kind(key: String, kind: &ValidationErrorsKind) -> Vec<(String, String)> {
    let mut errors = Vec::new();

    if let ValidationErrorsKind::Field(f) = kind {
        errors.push((
            key.clone(),
            f.iter()
                .map(ValidationError::to_string)
                .collect::<Vec<_>>()
                .join(","),
        ));
    }

    if let ValidationErrorsKind::List(l) = kind {
        for (i, e) in l {
            for (k, e) in e.errors() {
                errors.extend(build_kind(format!("{}[{}].{}", key, i, k), e));
            }
        }
    }

    if let ValidationErrorsKind::Struct(l) = kind {
        for (k, e) in l.as_ref().errors() {
            errors.extend(build_kind(format!("{}.{}", key, k), e));
        }
    }

    errors
}

impl ResponseError for ApiError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        let rest: RestError = self.into();
        ResponseError::status_code(&rest)
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        let rest: RestError = self.into();
        ResponseError::error_response(&rest)
    }
}
