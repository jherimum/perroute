use async_trait::async_trait;
use axum::extract::FromRequestParts;
use perroute_commons::types::actor::Actor;

pub struct ActorExtractor(pub Actor);

#[async_trait]
impl<S> FromRequestParts<S> for ActorExtractor {
    type Rejection = ();

    async fn from_request_parts(
        _: &mut axum::http::request::Parts,
        _: &S,
    ) -> Result<Self, Self::Rejection> {
        Ok(ActorExtractor(Actor::system()))
    }
}
