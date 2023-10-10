use strum_macros::Display;

#[derive(Display, Debug, Clone)]
pub enum CommandType {
    CreateBusinessUnit,
    UpdateBusinessUnit,
    DeleteBusinessUnit,

    CreateMessageType,
    UpdateMessageType,
    DeleteMessageType,

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
