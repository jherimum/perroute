use chrono::Utc;
use perroute_commons::{new_id, types::actor::Actor};
use perroute_messaging::events::IntoEvent;
use perroute_storage::models::command_log::{CommandLog, CommandLogBuilder};
use serde::Serialize;
use std::fmt::Debug;
use strum_macros::Display;

pub trait Command: Debug + Serialize + Clone + PartialEq + Eq + Send + Sync + IntoEvent {
    fn ty(&self) -> CommandType;

    fn to_log<E>(&self, actor: &Actor, error: Option<&E>) -> CommandLog
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        CommandLogBuilder::default()
            .id(new_id!())
            .actor_type(*actor.ty())
            .actor_id(*actor.id())
            .command_type(self.ty().to_string())
            .payload(serde_json::to_value(self).unwrap())
            .created_at(Utc::now().naive_utc())
            .error(error.map(|e| format!("{e}")))
            .build()
            .unwrap()
    }
}

#[derive(Display, Debug, Clone)]
pub enum CommandType {
    CreateBusinessUnit,
    UpdateBusinessUnit,
    DeleteBusinessUnit,

    CreateMessageType,
    UpdateMessageType,
    DeleteMessageType,

    CreateSchema,
    UpdateSchema,
    DeleteSchema,
    PublishSchema,

    CreateTemplate,
    UpdateTemplate,
    DeleteTemplate,

    CreateMessage,
    DistributeMessage,

    CreateConnection,
    DeleteConnection,
    UpdateConnection,

    CreateChannel,
    DeleteChannel,
    UpdateChannel,

    CreateRoute,
    DeleteRoute,
    UpdateRoute,

    ExecuteMessageDispatch,
}
