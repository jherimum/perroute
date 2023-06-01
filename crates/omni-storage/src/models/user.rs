use sqlx::PgExecutor;

pub struct User {
    pub id: uuid::Uuid,
    pub email: String,
}

impl User {
    pub fn new(email: impl Into<String>) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            email: email.into(),
        }
    }

    pub async fn save<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<User, sqlx::Error> {
        todo!()
    }
}
