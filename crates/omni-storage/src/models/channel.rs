use omni_commons::types::code::Code;
use sqlx::{FromRow, PgExecutor};
use tap::TapFallible;

#[derive(Debug, FromRow)]
pub struct Channel {
    pub id: uuid::Uuid,
    pub code: Code,
    pub name: String,
}

impl Channel {
    pub fn new(code: Code, name: impl Into<String>) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            code: code.into(),
            name: name.into(),
        }
    }

    pub async fn exists_by_code<'e, E: PgExecutor<'e>>(
        exec: E,
        code: &Code,
    ) -> Result<bool, sqlx::Error> {
        Ok(Self::find_by_code(exec, code)
            .await?
            .map_or_else(|| false, |_| true))
    }

    pub async fn save<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<Self, sqlx::Error> {
        sqlx::query_as("INSERT INTO channels (id, code, name) VALUES($1, $2, $3) RETURNING *")
            .bind(self.id)
            .bind(self.code)
            .bind(self.name)
            .fetch_one(exec)
            .await
            .tap_err(|e| tracing::error!("Query error. {e}"))
    }

    pub async fn update<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<Self, sqlx::Error> {
        sqlx::query_as("UPDATE channels SET name= $1 WHERE id= $2 RETURNING *")
            .bind(self.name)
            .bind(self.id)
            .fetch_one(exec)
            .await
            .tap_err(|e| tracing::error!("Query error. {e}"))
    }

    pub async fn delete<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<bool, sqlx::Error> {
        sqlx::query("DELETE FROM channels WHERE id= $1")
            .bind(self.id)
            .execute(exec)
            .await
            .tap_err(|e| tracing::error!("Query error. {e}"))
            .map(|result| result.rows_affected() > 0)
    }

    pub async fn find_by_id<'e, E: PgExecutor<'e>>(
        exec: E,
        id: &uuid::Uuid,
    ) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM channels WHERE id = $1")
            .bind(id)
            .fetch_optional(exec)
            .await
            .tap_err(|e| tracing::error!("Query error. {e}"))
    }

    pub async fn find_by_code<'e, E: PgExecutor<'e>>(
        exec: E,
        code: &Code,
    ) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM channels WHERE code = $1")
            .bind(code)
            .fetch_optional(exec)
            .await
            .tap_err(|e| tracing::error!("Query error. {e}"))
    }
}
