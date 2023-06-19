use super::id::Id;
use derive_getters::Getters;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, sqlx::Type, Copy)]
#[sqlx(type_name = "actor_type", rename_all = "snake_case")]
pub enum ActorType {
    User,
    System,
    Service,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Getters)]
pub struct Actor {
    id: Option<Id>,
    ty: ActorType,
}

impl Actor {
    pub fn new(id: Option<Id>, ty: ActorType) -> Self {
        Self { id, ty }
    }

    pub fn system() -> Self {
        Self::new(None, ActorType::System)
    }

    pub fn user(id: Id) -> Self {
        Self::new(Some(id), ActorType::User)
    }

    pub fn service(id: Id) -> Self {
        Self::new(Some(id), ActorType::Service)
    }
}
