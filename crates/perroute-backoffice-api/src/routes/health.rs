use actix_web::HttpResponse;

pub struct HealthRouter;

impl HealthRouter {
    #[tracing::instrument]
    pub async fn health() -> HttpResponse {
        HttpResponse::Ok().finish()
    }
}
