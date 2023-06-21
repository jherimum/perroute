use super::user::User;
use perroute_commons::types::id::Id;
use sqlx::PgExecutor;

pub struct UserPassword {
    id: Id,
    hash: String,
    user_id: Id,
}

impl UserPassword {
    pub fn new(user: &User, hash: impl Into<String>) -> Self {
        Self {
            id: Id::new(),
            hash: hash.into(),
            user_id: *user.id(),
        }
    }

    pub async fn save<'e, E: PgExecutor<'e>>(self, _exec: E) -> Result<UserPassword, sqlx::Error> {
        todo!()
    }
}
