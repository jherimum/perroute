use std::todo;

use sqlx::PgExecutor;

#[derive(Debug)]
pub struct Account {
    pub id: uuid::Uuid,
    pub code: String,
}

impl Account {
    pub fn new(code: impl Into<String>) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            code: code.into(),
        }
    }

    pub async fn find<'e, E: PgExecutor<'e>>(
        exec: E,
        id: &uuid::Uuid,
    ) -> Result<Option<Account>, sqlx::Error> {
        todo!()
    }

    pub async fn find_by_code<'e, E: PgExecutor<'e>>(
        exec: E,
        code: impl Into<String>,
    ) -> Result<Option<Account>, sqlx::Error> {
        todo!()
    }

    pub async fn save<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<Account, sqlx::Error> {
        todo!()
    }
}
