use sqlx::Executor;

#[derive(Debug)]
pub struct Channel {
    pub id: uuid::Uuid,
    pub code: String,
    pub description: Option<String>,
}

impl Channel {
    pub fn new(code: impl Into<String>, desc: Option<impl Into<String>>) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            code: code.into(),
            description: desc.map(Into::into),
        }
    }

    pub async fn exists_by_code<'e, E: Executor<'e>>(
        exec: E,
        code: impl Into<String>,
    ) -> Result<bool, sqlx::Error> {
        todo!()
    }

    pub async fn save<'e, E: Executor<'e>>(self, exec: E) -> Result<Self, sqlx::Error> {
        todo!()
    }

    pub async fn update<'e, E: Executor<'e>>(
        self,
        exec: E,
        desc: Option<String>,
    ) -> Result<Self, sqlx::Error> {
        todo!()
    }

    pub async fn delete<'e, E: Executor<'e>>(self, exec: E) -> Result<bool, sqlx::Error> {
        todo!()
    }

    pub async fn find<'e, E: Executor<'e>>(
        exec: E,
        id: &uuid::Uuid,
    ) -> Result<Option<Self>, sqlx::Error> {
        todo!()
    }
}
