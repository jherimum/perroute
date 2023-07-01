use actix_web::{
    web::{get, resource},
    HttpResponse, Scope,
};

pub struct HealthRouter;

impl HealthRouter {
    pub fn routes() -> Scope {
        Scope::new("/health").service(resource("").name("health").route(get().to(Self::health)))
    }

    #[tracing::instrument]
    pub async fn health() -> HttpResponse {
        HttpResponse::Ok().finish()
    }
}
