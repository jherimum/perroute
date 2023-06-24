use axum::Router;

use crate::rest::Buses;

pub struct MessageTypeVersionRouter;

impl MessageTypeVersionRouter {
    pub fn routes(buses: Buses) -> Router {
        Router::new()
    }
}
