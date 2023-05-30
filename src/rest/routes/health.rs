use axum::http::StatusCode;
use axum::routing::get;
use axum::Router;

pub fn routes() -> Router {
    Router::new().route("/", get(health))
}

#[tracing::instrument]
async fn health() -> StatusCode {
    StatusCode::OK
}
