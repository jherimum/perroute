use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};
use omni_connectors::{ConnectorPlugin, DispatcherPlugin};
use omni_cqrs::message_bus::MessageBus;
use std::todo;

use crate::rest::{
    api_models::plugin::{ConnectorPluginResource, Dispatcher},
    error::RestError,
};

pub fn routes(message_bus: MessageBus) -> Router {
    Router::new()
        .route("/", get(all))
        .route("/:id", get(find))
        .with_state(message_bus)
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Connector Plugin with Id {0} not found")]
    PluginNotFound(String),
}

impl From<Error> for RestError {
    fn from(value: Error) -> Self {
        match value {
            Error::PluginNotFound(_) => RestError::NotFound(value.to_string()),
        }
    }
}

async fn all(
    State(_message_bus): State<MessageBus>,
) -> Result<Json<Vec<ConnectorPluginResource>>, RestError> {
    // Ok(Json(
    //     plugins
    //         .all()
    //         .into_iter()
    //         .map(ConnectorPluginResource::from)
    //         .collect(),
    // ))
    todo!()
}

async fn find(
    State(_message_bus): State<MessageBus>,
    Path(_id): Path<String>,
) -> Result<Json<ConnectorPluginResource>, RestError> {
    // Ok(Json(
    //     plugins
    //         .get(&id)
    //         .ok_or(Error::PluginNotFound(id))
    //         .map(ConnectorPluginResource::from)?,
    // ))
    todo!()
}

impl From<&dyn ConnectorPlugin> for ConnectorPluginResource {
    fn from(value: &dyn ConnectorPlugin) -> Self {
        ConnectorPluginResource {
            id: value.id(),
            properties: value.configuration().properties.clone(),
            dispatchers: value
                .dispatchers()
                .values()
                .map(|d| Dispatcher::from(*d))
                .collect(),
        }
    }
}

impl From<&dyn DispatcherPlugin> for Dispatcher {
    fn from(value: &dyn DispatcherPlugin) -> Self {
        Dispatcher {
            type_: value.type_(),
            properties: value.configuration().properties.clone(),
        }
    }
}

// #[cfg(test)]
// mod tests {

//     use std::assert_eq;

//     use super::*;
//     use crate::{
//         connector::Plugins, cqrs::message_bus::MessageBusBuilder, rest::error::ErrorResponse,
//     };
//     use hyper::{Body, Method, Request, StatusCode};
//     use tower::ServiceExt;

//     #[tokio::test]
//     async fn test_when_not_found_plugin_by_id() {
//         let state = AppState::new(
//             crate::rest::Pool::Fake,
//             Plugins::builder().build(),
//             MessageBusBuilder::default().build(),
//         );
//         let req = Request::builder()
//             .uri("/Smtp")
//             .method(Method::GET)
//             .body(Body::empty())
//             .unwrap();
//         let response = super::routes(state).oneshot(req).await.unwrap();
//         assert_eq!(StatusCode::NOT_FOUND, response.status());

//         let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
//         let body: ErrorResponse = serde_json::from_slice(&body).unwrap();

//         let rest_error: RestError = Error::PluginNotFound("smtp".to_owned()).into();

//         assert_eq!(body, rest_error.as_error_response());
//     }
// }
