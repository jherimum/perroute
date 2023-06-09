use perroute_commons::types::email::Email;
use sqlx::{FromRow, PgExecutor};

#[derive(Debug, FromRow)]
pub struct User {
    pub id: uuid::Uuid,
    pub email: Email,
}

impl User {
    pub fn new(email: Email) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            email,
        }
    }

    pub async fn save<'e, E: PgExecutor<'e>>(self, _exec: E) -> Result<User, sqlx::Error> {
        todo!()
    }
}
