use actix_web::ResponseError;
use perroute_commons::{
    rest::RestError,
    types::{id::Id, json_schema::InvalidSchemaError},
};
use perroute_cqrs::{
    command_bus::{
        error::CommandBusError,
        handlers::business_unit::{
            create_business_unit::CreateBusinessUnitCommandHandlerError,
            update_business_unit::UpdateBusinessUnitCommandHandlerError,
        },
    },
    query_bus::error::QueryBusError,
};
use std::collections::HashMap;
use thiserror::Error;
use validator::{ValidationError, ValidationErrors, ValidationErrorsKind};

#[derive(Debug, Error)]
pub enum ApiError {
    #[error(transparent)]
    ValidationError(#[from] actix_web_validator::Error),

    #[error(transparent)]
    InvalidSchema(#[from] InvalidSchemaError),

    #[error(transparent)]
    CommandBus(#[from] CommandBusError),

    #[error(transparent)]
    QueryBus(#[from] QueryBusError),

    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),

    #[error(transparent)]
    Rest(#[from] RestError),
}

impl From<&ApiError> for RestError {
    fn from(value: &ApiError) -> Self {
        match value {
            ApiError::ValidationError(error) => match error {
                actix_web_validator::Error::Validate(e) => RestError::BadRequest(
                    Some("Invalid params".to_owned()),
                    validation_errors_to_hashmap(e),
                ),
                actix_web_validator::Error::Deserialize(_) => {
                    RestError::BadRequest(Some("Deserialization error".to_owned()), None)
                }
                actix_web_validator::Error::JsonPayloadError(e) => {
                    tracing::error!("Error that occur during reading payload: {e}");
                    RestError::BadRequest(Some("JsonPayloadError error".to_owned()), None)
                }
                actix_web_validator::Error::UrlEncodedError(e) => {
                    RestError::BadRequest(Some("UrlEncodedError error".to_owned()), None)
                }
                actix_web_validator::Error::QsError(e) => {
                    RestError::BadRequest(Some("QsError error".to_owned()), None)
                }
            },
            ApiError::CommandBus(CommandBusError::ExpectedError(message)) => {
                Self::UnprocessableEntity((*message).to_string())
            }
            ApiError::CommandBus(CommandBusError::CreateBusinessUnit(e)) => match e {
                CreateBusinessUnitCommandHandlerError::CodeAlreadyExists(_) => {
                    RestError::UnprocessableEntity(e.to_string())
                }
            },

            ApiError::CommandBus(CommandBusError::UpdateBusinessUnit(e)) => match e {
                UpdateBusinessUnitCommandHandlerError::BusinessUnitNotFound(_) => {
                    RestError::NotFound("Business unit not found".to_string())
                }
            },
            ApiError::QueryBus(QueryBusError::EntityNotFound(e)) => {
                RestError::NotFound(e.to_string())
            }
            ApiError::Rest(e) => e.clone(),
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
        map.extend(build_kind((*key).to_string(), kind));
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
                errors.extend(build_kind(format!("{key}[{i}].{k}"), e));
            }
        }
    }

    if let ValidationErrorsKind::Struct(l) = kind {
        for (k, e) in l.as_ref().errors() {
            errors.extend(build_kind(format!("{key}.{k}"), e));
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
