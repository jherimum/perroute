use perroute_commons::types::id::Id;
use perroute_storage::models::{api_key::ApiKey, user::User};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Actor {
    User(Id),
    System,
    Service(Id),
}

impl From<User> for Actor {
    fn from(user: User) -> Self {
        Actor::User(*user.id())
    }
}

impl From<ApiKey> for Actor {
    fn from(value: ApiKey) -> Self {
        Actor::Service(*value.id())
    }
}
