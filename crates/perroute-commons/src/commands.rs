use crate::impl_sqlx_type;

#[derive(Debug, Clone, PartialEq, Eq, strum::EnumString, strum::Display)]
pub enum CommandType {
    CreateBusinessUnit,
    UpdateBusinessUnit,
    DeleteBusinessUnit,
    CreateChannel,
    UpdateChannel,
    DeleteChannel,
    CreateMessageType,
    UpdateMessageType,
    DeleteMessageType,
    CreateRoute,
    UpdateRoute,
    DeleteRoute,
    CreateTemplateAssignment,
    UpdateTemplateAssignment,
    DeleteTemplateAssignment,
    CreateMessage,
}

impl_sqlx_type!(CommandType as String);
