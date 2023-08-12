use actix_web::FromRequest;
use futures::future::{ready, Ready};
use perroute_commons::{rest::RestError, types::actor::Actor};
pub struct ActorExtractor(pub Actor);

impl FromRequest for ActorExtractor {
    type Future = Ready<Result<Self, Self::Error>>;
    type Error = RestError;

    fn from_request(_: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        ready(Ok(Self(Actor::system())))
    }
}
