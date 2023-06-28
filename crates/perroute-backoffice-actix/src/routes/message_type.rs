use actix_web::{
    web::{delete, get, post, put, resource},
    HttpResponse, Responder, Scope,
};

pub struct MessageTypeRouter;

impl MessageTypeRouter {
    pub fn routes() -> Scope {
        Scope::new("/v1/channels/{channel_id}/message_types")
            .service(
                resource("")
                    .name("message_types")
                    .route(post().to(Self::create))
                    .route(get().to(Self::query)),
            )
            .service(
                resource("/{message_type_id}")
                    .name("message_type")
                    .route(get().to(Self::find))
                    .route(put().to(Self::update))
                    .route(delete().to(Self::delete)),
            )
    }

    #[tracing::instrument(name = "CHANNEL")]
    pub async fn query() -> impl Responder {
        HttpResponse::Ok().finish()
    }

    #[tracing::instrument(name = "CHANNEL")]
    pub async fn create() -> impl Responder {
        HttpResponse::Ok().finish()
    }

    #[tracing::instrument(name = "CHANNEL")]
    pub async fn update() -> impl Responder {
        HttpResponse::Ok().finish()
    }

    #[tracing::instrument(name = "CHANNEL")]
    pub async fn delete() -> impl Responder {
        HttpResponse::Ok().finish()
    }

    #[tracing::instrument(name = "CHANNEL")]
    pub async fn find() -> impl Responder {
        HttpResponse::Ok().finish()
    }
}
