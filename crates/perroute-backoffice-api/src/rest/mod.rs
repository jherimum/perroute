use axum::extract::FromRef;
use perroute_cqrs::{command_bus::bus::CommandBus, query_bus::bus::QueryBus};
use std::fmt::Debug;

pub mod api_models;
pub mod error;
pub mod extractors;
pub mod routes;

#[derive(Clone, Debug)]
pub struct Buses {
    pub command_bus: CommandBus,
    pub query_bus: QueryBus,
}

impl FromRef<Buses> for CommandBus {
    fn from_ref(input: &Buses) -> Self {
        input.command_bus.clone()
    }
}

impl FromRef<Buses> for QueryBus {
    fn from_ref(input: &Buses) -> Self {
        input.query_bus.clone()
    }
}

impl Buses {
    pub fn new(command_bus: CommandBus, query_bus: QueryBus) -> Self {
        Self {
            command_bus,
            query_bus,
        }
    }
}
