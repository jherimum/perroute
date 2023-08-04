use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, PartialEq, Eq, Clone, Getters, Deserialize, Serialize)]
pub struct Recipient {
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone_number: Option<String>,
}
