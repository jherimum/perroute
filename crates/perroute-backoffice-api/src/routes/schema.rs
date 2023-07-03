use actix_web::{
    web::{Data, Path},
    HttpResponse, Responder,
};
use perroute_commons::types::id::Id;

use crate::{app::AppState, extractors::actor::ActorExtractor};

pub const SCHEMAS_RESOURCE_NAME: &str = "schemas";
pub const SCHEMA_RESOURCE_NAME: &str = "schema";

pub struct SchemaRouter;

impl SchemaRouter {
    #[tracing::instrument]
    pub async fn query_shemas(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        schemas_path: Path<(Id, Id)>,
    ) -> impl Responder {
        HttpResponse::Ok().finish()
    }

    #[tracing::instrument]
    pub async fn create_schema(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        schemas_path: Path<(Id, Id)>,
    ) -> impl Responder {
        HttpResponse::Ok().finish()
    }

    #[tracing::instrument]
    pub async fn update_schema(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        schema_path: Path<(Id, Id, Id)>,
    ) -> impl Responder {
        HttpResponse::Ok().finish()
    }

    #[tracing::instrument]
    pub async fn delete_schema(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        schema_path: Path<(Id, Id, Id)>,
    ) -> impl Responder {
        HttpResponse::Ok().finish()
    }

    #[tracing::instrument]
    pub async fn find_schema(
        state: Data<AppState>,
        ActorExtractor(actor): ActorExtractor,
        schema_path: Path<(Id, Id, Id)>,
    ) -> impl Responder {
        HttpResponse::Ok().finish()
    }
}
