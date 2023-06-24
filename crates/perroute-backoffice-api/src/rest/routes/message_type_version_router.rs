use crate::rest::Buses;
use axum::{
    response::IntoResponse,
    routing::{delete, get, post, put},
    Router,
};

pub struct MessageTypeVersionRouter;

impl MessageTypeVersionRouter {
    pub fn routes(buses: Buses) -> Router {
        Router::new()
            .nest(
                "/v1/channels/:channel_id/message_types/:message_type_id/versions",
                Router::new()
                    .route("/", get(Self::query_versions))
                    .route("/", post(Self::create_version))
                    .nest(
                        "/:message_type_version_id",
                        Router::new()
                            .route("/", get(Self::find_version))
                            .route("/", put(Self::update_version))
                            .route("/", delete(Self::delete_version))
                            .route("/publish", post(Self::publish_version))
                            .route("/duplicate", post(Self::duplicate_version)),
                    ),
            )
            .with_state(buses)
    }

    async fn query_versions() -> impl IntoResponse {
        todo!()
    }

    async fn create_version() -> impl IntoResponse {
        todo!()
    }

    async fn update_version() -> impl IntoResponse {
        todo!()
    }

    async fn delete_version() -> impl IntoResponse {
        todo!()
    }

    async fn find_version() -> impl IntoResponse {
        todo!()
    }

    async fn publish_version() -> impl IntoResponse {
        todo!()
    }

    async fn duplicate_version() -> impl IntoResponse {
        todo!()
    }
}
