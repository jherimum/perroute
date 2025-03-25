use serde::{Deserialize, Serialize};

use crate::impl_sqlx_type;

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    strum::EnumString,
    strum::Display,
    Serialize,
    Deserialize,
    Hash,
    Copy,
)]
pub enum DispatchType {
    Email,
    Sms,
    Push,
}

impl_sqlx_type!(DispatchType as String);
