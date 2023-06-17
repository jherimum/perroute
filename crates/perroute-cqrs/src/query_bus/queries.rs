use strum_macros::Display;

pub mod channel;

#[derive(Debug, Clone, PartialEq, Eq, Display)]
pub enum QueryType {
    FindChannel,
}
