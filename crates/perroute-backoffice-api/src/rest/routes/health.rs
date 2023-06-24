use axum::http::StatusCode;
use axum::routing::get;
use axum::Router;

pub struct HealthRouter;

impl HealthRouter {
    pub fn routes() -> Router {
        Router::new().route("/health", get(Self::health))
    }

    #[tracing::instrument]
    async fn health() -> StatusCode {
        StatusCode::OK
    }
}
