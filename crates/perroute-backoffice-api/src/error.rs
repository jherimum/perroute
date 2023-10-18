use actix_web::{body::BoxBody, HttpResponse, ResponseError};
use http::StatusCode;
use perroute_commandbus::{
    command::{
        business_unit::{
            create_business_unit::CreateBusinessUnitError,
            update_business_unit::UpdateBusinessUnitError,
        },
        connection::{
            delete_connection::DeleteConnectionError, update_connection::UpdateConnectionError,
        },
    },
    error::CommandBusError,
};
use perroute_commons::types::json_schema::InvalidSchemaError;
use perroute_cqrs::query_bus::error::QueryBusError;
use serde::{Deserialize, Serialize};
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
                    validation_errors_to_hashmap(&e),
                ),
                actix_web_validator::Error::Deserialize(_) => {
                    RestError::BadRequest(Some("Deserialization error".to_owned()), None)
                }
                actix_web_validator::Error::JsonPayloadError(e) => {
                    tracing::error!("Error that occur during reading payload: {e}");
                    RestError::BadRequest(Some("JsonPayloadError error".to_owned()), None)
                }
                actix_web_validator::Error::UrlEncodedError(_) => {
                    RestError::BadRequest(Some("UrlEncodedError error".to_owned()), None)
                }
                actix_web_validator::Error::QsError(_) => {
                    RestError::BadRequest(Some("QsError error".to_owned()), None)
                }
            },
            ApiError::CommandBus(CommandBusError::CreateBusinessUnit(e)) => match e {
                CreateBusinessUnitError::CodeAlreadyExists(_) => {
                    RestError::UnprocessableEntity(e.to_string())
                }
            },

            ApiError::CommandBus(CommandBusError::UpdateBusinessUnit(e)) => match e {
                UpdateBusinessUnitError::BusinessUnitNotFound(_) => {
                    RestError::NotFound("Business unit not found".to_string())
                }
            },
            //connection
            ApiError::CommandBus(CommandBusError::UpdateConnection(e)) => match e {
                UpdateConnectionError::ConnectionNotFound(_) => {
                    RestError::NotFound("Business unit not found".to_string())
                }
                UpdateConnectionError::InvalidProperties(e) => {
                    RestError::UnprocessableEntity(e.to_string())
                }
            },

            ApiError::CommandBus(CommandBusError::CreateConnection(e)) => match e {
                perroute_commandbus::command::connection::create_connection::CreateConnectionError::PluginNotFound(_) => {
                    RestError::BadRequest(Some("Plugin do not exists".to_owned()), None)
                },
                perroute_commandbus::command::connection::create_connection::CreateConnectionError::InvalidProperties(_) => {
                    RestError::BadRequest(Some("Invalid properties".to_owned()), None)
                },
            },

            ApiError::CommandBus(CommandBusError::DeleteConnection(e)) => match e {
                DeleteConnectionError::ConnectionNotFound(_) => RestError::NotFound(e.to_string()),
                DeleteConnectionError::DeleteError(_, _) => RestError::UnprocessableEntity(e.to_string()),
            },

            //query bus
            ApiError::QueryBus(QueryBusError::EntityNotFound(e)) => {
                RestError::NotFound(e.to_string())
            },

            ApiError::Rest(e) => e.clone(),

            e => Self::InternalServer(Some(e.to_string())),
        }
    }
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

impl ResponseError for RestError {
    fn status_code(&self) -> StatusCode {
        self.status_code()
    }
    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code()).json(self.model())
    }
}

#[derive(Debug, thiserror::Error, Clone)]
pub enum RestError {
    #[error("{0}")]
    NotFound(String),

    #[error("Internal Server Error")]
    InternalServer(Option<String>),

    #[error("{0}")]
    UnprocessableEntity(String),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Forbidden")]
    Forbidden,

    #[error("BadRequest")]
    BadRequest(Option<String>, Option<HashMap<String, String>>),
}

impl RestError {
    fn status_code(&self) -> StatusCode {
        match self {
            RestError::NotFound(_) => StatusCode::NOT_FOUND,
            RestError::InternalServer(_) => StatusCode::INTERNAL_SERVER_ERROR,
            RestError::UnprocessableEntity(_) => StatusCode::UNPROCESSABLE_ENTITY,
            RestError::Unauthorized => StatusCode::UNAUTHORIZED,
            RestError::Forbidden => StatusCode::FORBIDDEN,
            RestError::BadRequest(_, _) => StatusCode::BAD_REQUEST,
        }
    }

    fn model(&self) -> ErrorModel {
        let message = self.to_string();
        let status_code = self.status_code();
        match self {
            RestError::NotFound(detail) => {
                ErrorModel::new(status_code, message, Some(detail.to_owned()), None)
            }
            RestError::InternalServer(e) => {
                ErrorModel::new(status_code, message, e.to_owned(), None)
            }
            RestError::UnprocessableEntity(detail) => {
                ErrorModel::new(status_code, message, Some(detail.to_owned()), None)
            }
            RestError::Unauthorized => ErrorModel::new(status_code, message, None, None),
            RestError::Forbidden => ErrorModel::new(status_code, message, None, None),
            RestError::BadRequest(detail, errors) => {
                ErrorModel::new(status_code, message.clone(), detail.clone(), errors.clone())
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ErrorModel {
    pub status: u16,
    pub message: String,
    pub detail: Option<String>,
    pub errors: Option<HashMap<String, String>>,
}

impl ErrorModel {
    pub fn new(
        status: StatusCode,
        message: String,
        detail: Option<String>,
        errors: Option<HashMap<String, String>>,
    ) -> Self {
        Self {
            status: status.into(),
            message,
            detail,
            errors,
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
