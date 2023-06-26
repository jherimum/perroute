use serde_json::Value;
use std::ops::Deref;

pub struct Payload(Value);

impl Payload {
    pub fn new(value: Value) -> Self {
        Payload(value)
    }
}

impl Deref for Payload {
    type Target = Value;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
