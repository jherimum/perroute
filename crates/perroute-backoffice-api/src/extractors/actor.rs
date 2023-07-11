use actix_web::FromRequest;
use perroute_commons::types::actor::Actor;
use std::future::{ready, Ready};

pub struct ActorExtractor(pub Actor);

impl FromRequest for ActorExtractor {
    type Future = Ready<Result<Self, Self::Error>>;
    type Error = actix_web::Error;

    fn from_request(_: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        ready(Ok(ActorExtractor(Actor::system())))
    }
}
