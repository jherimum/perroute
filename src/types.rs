use crate::errors::OmniMessageError;

pub type OmniResult<T> = std::result::Result<T, OmniMessageError>;
