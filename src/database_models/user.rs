use sqlx::PgExecutor;

use super::account::Account;

pub struct User {
    pub id: uuid::Uuid,
    pub email: String,
    pub account_id: uuid::Uuid,
}

impl User {
    pub fn new(email: impl Into<String>, account: &Account) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            email: email.into(),
            account_id: account.id,
        }
    }

    pub async fn save<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<User, sqlx::Error> {
        todo!()
    }
}
