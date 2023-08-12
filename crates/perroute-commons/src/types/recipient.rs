use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, PartialEq, Eq, Clone, Getters, Deserialize, Serialize)]
pub struct Recipient {
    pub name: String,
    pub email: String,
    pub phone_number: String,
}
