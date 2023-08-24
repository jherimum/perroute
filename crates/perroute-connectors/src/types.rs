use serde::{Deserialize, Serialize};
use sqlx::Type;
pub mod delivery;
pub mod dispatch_type;
pub mod plugin_id;

#[derive(Debug, Deserialize, PartialEq, Eq, Copy, Clone, Serialize, Type)]
pub enum TemplateSupport {
    Mandatory,
    Optional,
    None,
}
