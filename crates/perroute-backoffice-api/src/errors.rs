use perroute_commons::rest::RestError;
use perroute_cqrs::{
    command_bus::{
        error::CommandBusError,
        handlers::channel::{
            create_channel::CreateChannelError, delete_channel::DeleteChannelError,
            update_channel::UpdateChannelError,
        },
    },
    query_bus::error::QueryBusError,
};

#[derive(Debug, thiserror::Error)]
pub enum PerrouteBackofficeApiError {
    #[error(transparent)]
    CommandBus(#[from] CommandBusError),

    #[error(transparent)]
    QueryBus(#[from] QueryBusError),
}

impl From<PerrouteBackofficeApiError> for RestError {
    fn from(value: PerrouteBackofficeApiError) -> Self {
        match value {
            PerrouteBackofficeApiError::CommandBus(e) => match e {
                CommandBusError::CreateChannel(e) => match e {
                    CreateChannelError::CodeAlreadyExists(_) => {
                        RestError::UnprocessableEntity(e.to_string())
                    }
                },
                CommandBusError::UpdateChannel(e) => match e {
                    UpdateChannelError::ChannelNotFound(_) => RestError::NotFound(e.to_string()),
                },
                CommandBusError::DeleteChannel(e) => match e {
                    DeleteChannelError::ChannelNotFound(_) => RestError::NotFound(e.to_string()),
                },
                _ => RestError::InternalServer,
            },
            PerrouteBackofficeApiError::QueryBus(_) => RestError::InternalServer,
        }
    }
}
