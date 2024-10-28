use perroute_commons::types::{id::Id, Code, Name, Timestamp};
use sqlx::prelude::FromRow;

#[derive(Debug, Clone, PartialEq, Eq, FromRow)]
pub struct BusinessUnit {
    pub id: Id,
    pub code: Code,
    pub name: Name,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}
