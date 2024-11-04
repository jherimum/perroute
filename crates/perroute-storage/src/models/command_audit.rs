use bon::builder;
use derive_getters::Getters;
use derive_setters::Setters;
use perroute_commons::{
    commands::CommandType,
    types::{
        actor::{Actor, ActorType},
        id::Id,
        Timestamp,
    },
};
use serde::Serialize;
use sqlx::prelude::FromRow;

#[derive(Debug, Clone, PartialEq, Eq, FromRow, Getters, Setters)]
#[setters(prefix = "set_")]
#[setters(into)]
pub struct CommandAudit {
    id: Id,
    command_type: CommandType,
    command_data: serde_json::Value,
    actor_type: ActorType,
    actor_id: Option<Id>,
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
        actor_type: actor.actor_type(),
        actor_id: actor.id().cloned(),
        created_at: Timestamp::now(),
    }
}

impl perroute_commons::types::entity::Entity for CommandAudit {
    fn id(&self) -> &Id {
        &self.id
    }
}
