use crate::{connector::Plugins, cqrs::message_bus::MessageBus};
use axum::extract::FromRef;
use sqlx::PgPool;

pub mod api_models;
pub mod error;
pub mod extractors;
pub mod routes;

// #[derive(Clone)]
// pub enum Pool {
//     Real(PgPool),
//     Fake,
// }

// #[derive(Clone)]
// pub struct AppState {
//     pool: Pool,
//     plugins: Plugins,
//     message_bus: MessageBus,
// }

// impl AppState {
//     pub fn new(pool: Pool, plugins: Plugins, message_bus: MessageBus) -> AppState {
//         Self {
//             pool,
//             plugins,
//             message_bus,
//         }
//     }
// }

// impl FromRef<AppState> for Pool {
//     fn from_ref(app_state: &AppState) -> Pool {
//         app_state.pool.clone()
//     }
// }

// impl FromRef<AppState> for Plugins {
//     fn from_ref(app_state: &AppState) -> Plugins {
//         app_state.plugins.clone()
//     }
// }
