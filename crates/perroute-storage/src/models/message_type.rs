use perroute_commons::types::{id::Id, Code, Name, Schema, Timestamp, Vars};
use sqlx::prelude::FromRow;

#[derive(Debug, Clone, PartialEq, Eq, FromRow)]
pub struct MessageType {
    pub id: Id,
    pub code: Code,
    pub name: Name,
    pub vars: Vars,
    pub schema: Schema,
    pub enabled: bool,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}
