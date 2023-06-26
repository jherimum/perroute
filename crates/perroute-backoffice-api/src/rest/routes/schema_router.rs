use crate::rest::Buses;
use axum::{
    response::IntoResponse,
    routing::{delete, get, post, put},
    Router,
};

pub struct SchemaRouter;

impl SchemaRouter {
    pub fn routes(buses: Buses) -> Router {
        Router::new()
            .nest(
                "/v1/channels/:channel_id/message_types/:message_type_id/schemas",
                Router::new()
                    .route("/", get(Self::query_schemas))
                    .route("/", post(Self::create_schema))
                    .nest(
                        "/:version",
                        Router::new()
                            .route("/", get(Self::find_schema))
                            .route("/", put(Self::update_schema))
                            .route("/", delete(Self::delete_schema))
                            .route("/publish", post(Self::publish_schema)),
                    ),
            )
            .with_state(buses)
    }

    async fn query_schemas() -> impl IntoResponse {
        todo!()
    }

    async fn create_schema() -> impl IntoResponse {
        todo!()
    }

    async fn update_schema() -> impl IntoResponse {
        todo!()
    }

    async fn delete_schema() -> impl IntoResponse {
        todo!()
    }

    async fn find_schema() -> impl IntoResponse {
        todo!()
    }

    async fn publish_schema() -> impl IntoResponse {
        todo!()
    }
}
