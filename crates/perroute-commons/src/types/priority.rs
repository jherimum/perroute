use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Debug, PartialEq, Eq, Clone, Type, Serialize, Deserialize, Copy)]
#[sqlx(transparent)]
pub struct Priority(i32);
