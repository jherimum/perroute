use bon::Builder;
use derive_getters::Getters;
use derive_setters::Setters;
use perroute_commons::types::{id::Id, vars::Vars, Code, Name, Schema, Timestamp};
use sqlx::prelude::FromRow;

#[derive(Debug, Clone, PartialEq, Eq, FromRow, Builder, Getters, Setters)]
#[setters(prefix = "set_")]
#[setters(into)]
pub struct MessageType {
    #[setters(skip)]
    pub id: Id,
    #[setters(skip)]
    pub code: Code,
    
    pub name: Name,
        
    pub vars: Option<Vars>,
    
    pub schema: Schema,
    
    pub enabled: bool,

    #[setters(skip)]
    pub created_at: Timestamp,
    
    pub updated_at: Timestamp,
}
