use crate::types::OmniResult;

use super::user::User;
use sqlx::PgExecutor;
use time::OffsetDateTime;

pub struct UserPassword {
    id: uuid::Uuid,
    hash: String,
    user_id: uuid::Uuid,
    created_at: OffsetDateTime,
}

impl UserPassword {
    pub fn new(user: &User, hash: impl Into<String>) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            hash: hash.into(),
            user_id: user.id,
            created_at: OffsetDateTime::now_utc(),
        }
    }

    pub async fn save<'e, E: PgExecutor<'e>>(self, exec: E) -> OmniResult<UserPassword> {
        todo!()
    }
}
