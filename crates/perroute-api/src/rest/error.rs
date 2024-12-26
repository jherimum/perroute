use actix_web::{
    body::BoxBody, http::StatusCode, HttpResponse, Responder, ResponseError,
};
use perroute_command_bus::CommandBusError;
use perroute_commons::types::{
    code::InvalidCodeError, name::InvalidNameError, schema::InvalidSchemaError,
};
use perroute_query_bus::QueryBusError;
use serde::{ser::SerializeStruct, Serialize};
use serde_json::Value;
use std::{collections::HashMap, error::Error};
use strum::ParseError;
use validator::{ValidationError, ValidationErrors, ValidationErrorsKind};

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Resource not found")]
    NotFound,

    #[error("Internal server error: {0}")]
    InternalServerError(#[from] Box<dyn Error>),

    #[error("Conflict")]
    Conflict,

    #[error("Bad request")]
    BadRequest,

    #[error("Command bus error: {0}")]
    CommandBusError(#[from] CommandBusError),

    #[error("Query bus error: {0}")]
    QueryBusError(#[from] QueryBusError),

    #[error("Json payload error: {0}")]
    JsonPayloadError(#[from] actix_web::error::JsonPayloadError),

    #[error("Path error: {0}")]
    PathError(#[from] actix_web::error::PathError),

    #[error("Query payload error: {0}")]
    QueryPayloadError(#[from] actix_web::error::QueryPayloadError),

    #[error("Form error: {0}")]
    ActixValidationError(#[from] actix_web_validator::Error),

    #[error("Invalid name error: {0}")]
    InvalidNameError(#[from] InvalidNameError),

    #[error("Invalid code error: {0}")]
    InvalidCodeError(#[from] InvalidCodeError),

    #[error("Invalid schema error: {0}")]
    InvalidSchemaError(#[from] InvalidSchemaError),

    #[error("Enum parser error: {0}")]
    EnumParserError(#[from] ParseError),
}

impl ResponseError for ApiError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        self.error_response().status()
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        let error = RestError::from(self);
        HttpResponse::build(error.status).json(error)
    }
}

impl From<&ApiError> for RestError {
    fn from(value: &ApiError) -> Self {
        match value {
            ApiError::NotFound => RestError::not_found("Resource not found"),
            ApiError::InternalServerError(_) => {
                RestError::internal_server("Internal server error")
            }
            ApiError::Conflict => RestError::conflict("Conflict"),
            ApiError::BadRequest => {
                RestError::bad_request("Bad request", Default::default())
            }
            ApiError::CommandBusError(e) => {
                RestError::internal_server(e.to_string())
            }
            ApiError::QueryBusError(e) => {
                RestError::internal_server(e.to_string())
            }
            ApiError::JsonPayloadError(ref e) => {
                RestError::bad_request(e.to_string(), Default::default())
            }
            ApiError::PathError(ref e) => {
                RestError::bad_request(e.to_string(), Default::default())
            }
            ApiError::QueryPayloadError(ref e) => {
                RestError::bad_request(e.to_string(), Default::default())
            }
            ApiError::ActixValidationError(error) => match error {
                actix_web_validator::Error::Validate(_) => todo!(),
                actix_web_validator::Error::Deserialize(_) => todo!(),
                actix_web_validator::Error::JsonPayloadError(_) => todo!(),
                actix_web_validator::Error::UrlEncodedError(_) => todo!(),
                actix_web_validator::Error::QsError(_) => todo!(),
            },
            _ => RestError::internal_server("Unknown error"),
        }
    }
}

#[derive(Debug)]
pub struct RestError {
    status: StatusCode,
    message: String,
    detail: Option<String>,
    errors: FieldErrors,
}

impl RestError {
    pub fn not_found<T: ToString>(detail: T) -> Self {
        Self {
            status: StatusCode::NOT_FOUND,
            message: "Resource not found".to_string(),
            detail: Some(detail.to_string()),
            errors: Default::default(),
        }
    }

    pub fn internal_server<T: Into<String>>(detail: T) -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: "Internal server error".to_string(),
            detail: Some(detail.into()),
            errors: Default::default(),
        }
    }

    pub fn conflict<T: ToString>(message: T) -> Self {
        Self {
            status: StatusCode::CONFLICT,
            message: message.to_string(),
            detail: None,
            errors: Default::default(),
        }
    }

    pub fn bad_request<T: ToString>(detail: T, errors: FieldErrors) -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            message: "Bad request".to_string(),
            detail: Some(detail.to_string()),
            errors,
        }
    }
}

impl Serialize for RestError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let mut state = serializer.serialize_struct("ErrorModel", 4)?;
        state.serialize_field("status", &self.status.as_u16())?;
        state.serialize_field("message", &self.message)?;
        state.serialize_field("detail", &self.detail)?;
        state.serialize_field("errors", &self.errors)?;
        state.end()
    }
}

impl Responder for RestError {
    type Body = BoxBody;

    fn respond_to(
        self,
        _: &actix_web::HttpRequest,
    ) -> HttpResponse<Self::Body> {
        HttpResponse::build(self.status).json(self)
    }
}

#[derive(serde::Serialize, Debug, Default)]
pub struct FieldErrors(Vec<FieldError>);

impl From<&ValidationErrors> for FieldErrors {
    fn from(value: &ValidationErrors) -> Self {
        let mut errors = Vec::new();
        value
            .errors()
            .iter()
            .for_each(|(key, kind)| build_errors(&mut errors, key, kind));

        Self(errors)
    }
}

#[derive(serde::Serialize, Debug)]
pub struct FieldError {
    path: String,
    code: String,
    message: Option<String>,
    params: Option<HashMap<String, Value>>,
}

impl FieldError {
    pub fn from(path: &str, error: ValidationError) -> Self {
        Self {
            path: path.to_string(),
            code: error.code.to_string(),
            message: error.message.as_ref().map(|x| x.to_string()),
            params: error
                .params
                .into_iter()
                .map(|(k, v)| (k.to_string(), v))
                .collect::<HashMap<_, _>>()
                .into(),
        }
    }
}

fn build_errors(
    errors: &mut Vec<FieldError>,
    key: &str,
    kind: &ValidationErrorsKind,
) {
    match kind {
        ValidationErrorsKind::Field(field_error) => {
            errors.extend(
                field_error.iter().map(|e| FieldError::from(key, e.clone())),
            );
        }
        ValidationErrorsKind::List(l) => {
            for (i, e) in l {
                for (k, e) in e.errors() {
                    build_errors(errors, format!("{key}[{i}].{k}").as_str(), e);
                }
            }
        }
        ValidationErrorsKind::Struct(l) => {
            for (k, e) in l.as_ref().errors() {
                build_errors(errors, format!("{key}.{k}").as_str(), e);
            }
        }
    }
}
