use perroute_commons::types::{id::Id, Email, Password, Timestamp, Username};
use sqlx::prelude::FromRow;

#[derive(Debug, Clone, PartialEq, Eq, FromRow)]
pub struct User {
    pub id: Id,
    pub username: Username,
    pub email: Email,
    pub password: Password,
    pub active: bool,
    pub superuser: bool,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

impl perroute_commons::types::entity::Entity for User {
    fn id(&self) -> &Id {
        &self.id
    }
}
