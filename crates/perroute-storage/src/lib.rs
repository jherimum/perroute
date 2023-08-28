pub mod connection_manager;
pub mod error;
pub mod models;
pub mod query;
pub mod utils;

#[macro_use]
mod macros {
    #[macro_export]
    macro_rules! log_query_error {
        () => {
            |e| tracing::error!("Query error. {e}")
        };
    }
}

pub trait DatabaseModel {}
