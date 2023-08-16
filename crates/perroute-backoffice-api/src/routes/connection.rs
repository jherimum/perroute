use crate::{
    api::models::connection::CreateConnectionRequest, app::AppState,
    extractors::actor::ActorExtractor,
};
use actix_web::{
    web::{Data, Json},
    HttpResponse,
};

pub struct ConnectionsRouter;

impl ConnectionsRouter {
    pub const CONN_RESOURCE_NAME: &str = "connection";
    pub const CONNS_RESOURCE_NAME: &str = "connections";

    pub async fn create(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        Json(body): Json<CreateConnectionRequest>,
    ) -> HttpResponse {
        HttpResponse::Ok().finish()
    }
}
