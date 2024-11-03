use std::ops::Deref;

use sqlx::prelude::Type;

#[derive(Debug, Clone, PartialEq, Eq, Type)]
#[sqlx(transparent)]
pub struct Priority(i64);

impl Priority {
    pub fn new(priority: i64) -> Self {
        Self(priority)
    }
}

impl Deref for Priority {
    type Target = i64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
