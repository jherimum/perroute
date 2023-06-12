use super::user::User;
use perroute_commons::types::id::Id;
use sqlx::PgExecutor;
use time::OffsetDateTime;

pub struct UserPassword {
    id: Id,
    hash: String,
    user_id: Id,
    created_at: OffsetDateTime,
}

impl UserPassword {
    pub fn new(user: &User, hash: impl Into<String>) -> Self {
        Self {
            id: Id::new(),
            hash: hash.into(),
            user_id: *user.id(),
            created_at: OffsetDateTime::now_utc(),
        }
    }

    pub async fn save<'e, E: PgExecutor<'e>>(self, _exec: E) -> Result<UserPassword, sqlx::Error> {
        todo!()
    }
}
