use perroute_commons::types::{actor::Actor, id::Id};
use serde::Serialize;
use sqlx::PgExecutor;

#[derive(Debug, Serialize, sqlx::Type)]
#[serde(transparent)]
#[sqlx(transparent)]
pub struct CommandPayload<C: Serialize + Clone>(C);

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct CommandLog<C>
where
    C: Serialize + Clone,
{
    id: Id,
    command_name: String,
    payload: CommandPayload<C>,
    actor: Actor,
    created_at: time::OffsetDateTime,
}

impl<C> CommandLog<C>
where
    C: Serialize + Clone,
{
    pub fn new(cmd_name: impl Into<String>, payload: &C, actor: &Actor) -> Self {
        Self {
            id: Id::new(),
            command_name: cmd_name.into(),
            payload: CommandPayload(payload.clone()),
            actor: actor.clone(),
            created_at: time::OffsetDateTime::now_utc(),
        }
    }

    pub async fn save<'e, E: PgExecutor<'e>>(self, exec: E) -> Result<Self, sqlx::Error> {
        // sqlx::query_as("INSERT INTO channels (id, code, name) VALUES($1, $2, $3) RETURNING *")
        //     .bind(self.id)
        //     .bind(self.code)
        //     .bind(self.name)
        //     .fetch_one(exec)
        //     .await
        //     .tap_err(|e| tracing::error!("Query error. {e}"))
        todo!()
    }
}
