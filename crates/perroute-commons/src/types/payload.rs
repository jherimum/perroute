use serde_json::Value;
use std::ops::Deref;

pub struct Payload(Value);

impl Payload {
    pub const fn new(value: Value) -> Self {
        Self(value)
    }
}

impl Deref for Payload {
    type Target = Value;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
