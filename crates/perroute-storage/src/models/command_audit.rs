use bon::builder;
use derive_getters::Getters;
use derive_setters::Setters;
use perroute_commons::{
    commands::CommandType,
    types::{actor::Actor, id::Id, Timestamp},
};
use serde::Serialize;
use sqlx::{prelude::FromRow, types::Json};

#[derive(Debug, Clone, PartialEq, Eq, FromRow, Getters, Setters)]
#[setters(prefix = "set_")]
#[setters(into)]
pub struct CommandAudit {
    id: Id,
    command_type: CommandType,
    command_data: serde_json::Value,
    actor: Json<Actor>,
    created_at: Timestamp,
}

#[builder]
pub fn command_audit_builder<D: Serialize>(
    command_type: &CommandType,
    command_data: &D,
    actor: &Actor,
) -> CommandAudit {
    CommandAudit {
        id: Id::new(),
        command_type: command_type.clone(),
        command_data: serde_json::to_value(command_data).unwrap(),
        actor: Json(actor.clone()),
        created_at: Timestamp::now(),
    }
}
