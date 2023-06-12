use derive_getters::Getters;
use perroute_commons::types::id::Id;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Getters)]
pub struct ApiKey {
    id: Id,
}
