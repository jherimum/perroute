use actix_web::{
    web::{get, resource},
    HttpResponse, Responder, Scope,
};

pub struct HealthRouter;

impl HealthRouter {
    pub fn routes() -> Scope {
        Scope::new("/health").service(resource("").name("health").route(get().to(Self::health)))
    }

    #[tracing::instrument(name = "HEALTH")]
    pub async fn health() -> impl Responder {
        HttpResponse::Ok().finish()
    }
}
