use axum::Router;
use omni_message::{rest::routes::health, tracing as omni_tracing};
use std::net::SocketAddr;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

#[tokio::main]
async fn main() {
    omni_tracing::init();

    let app = Router::new().nest("/healh", health::routes());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
