pub mod business_unit;
pub mod channel;
pub mod message;
pub mod message_type;
pub mod route;
pub mod template_assignment;

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
