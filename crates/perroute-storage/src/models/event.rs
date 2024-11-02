use bon::Builder;
use derive_getters::Getters;
use derive_setters::Setters;
use perroute_events::ApplicationEvent;
use sqlx::prelude::FromRow;

#[derive(Debug, Clone, PartialEq, Eq, FromRow, Builder, Getters, Setters)]
#[setters(prefix = "set_")]
#[setters(into)]
pub struct Event {}

impl<E: ApplicationEvent> From<E> for Event {
    fn from(event: E) -> Self {
        Event {}
    }
}
