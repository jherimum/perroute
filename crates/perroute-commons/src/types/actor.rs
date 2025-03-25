use serde::{Deserialize, Serialize};

use crate::impl_sqlx_type;

use super::id::Id;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Actor {
    User(Id),
    System,
    Service(Id),
}

impl Actor {
    pub fn id(&self) -> Option<&Id> {
        match self {
            Actor::User(id) => Some(id),
            Actor::Service(id) => Some(id),
            _ => None,
        }
    }

    pub fn actor_type(&self) -> ActorType {
        match self {
            Actor::User(_) => ActorType::User,
            Actor::System => ActorType::System,
            Actor::Service(_) => ActorType::Service,
        }
    }
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    strum::Display,
    strum::EnumString,
)]
pub enum ActorType {
    User,
    System,
    Service,
}

impl_sqlx_type!(ActorType as String);
