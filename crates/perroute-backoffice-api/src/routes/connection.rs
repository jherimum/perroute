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
    pub async fn create(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        Json(body): Json<CreateConnectionRequest>,
    ) -> HttpResponse {
        HttpResponse::Ok().finish()
    }
}
