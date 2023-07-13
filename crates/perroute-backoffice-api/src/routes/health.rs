use actix_web::HttpResponse;

pub struct HealthRouter;

impl HealthRouter {
    pub const HEALTH_RESOURCE_NAME: &str = "health";

    #[tracing::instrument]
    pub async fn health() -> HttpResponse {
        HttpResponse::Ok().finish()
    }
}
