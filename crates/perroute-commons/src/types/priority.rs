use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use sqlx::Type;
use validator::ValidationError;

#[derive(Debug, PartialEq, Eq, Clone, Type, Serialize, Deserialize, Copy)]
#[sqlx(transparent)]
pub struct Priority(i32);

impl Priority {
    pub fn validate(priority: i32) -> Result<(), ValidationError> {
        if priority < 1 {
            return Err(ValidationError {
                code: Cow::Borrowed("priority"),
                message: Some(Cow::Borrowed("Invalid priority")),
                params: Default::default(),
            });
        }

        Ok(())
    }
}

impl From<i32> for Priority {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl From<&i32> for Priority {
    fn from(priority: &i32) -> Self {
        Self(*priority)
    }
}

impl Into<i32> for Priority {
    fn into(self) -> i32 {
        self.0
    }
}

impl Into<i32> for &Priority {
    fn into(self) -> i32 {
        self.0
    }
}
