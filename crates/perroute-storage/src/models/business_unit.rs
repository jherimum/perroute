use bon::Builder;
use derive_getters::Getters;
use derive_setters::Setters;
use perroute_commons::types::{
    code::Code, entity::Entity, id::Id, name::Name, vars::Vars, Timestamp,
};
use sqlx::{prelude::FromRow, types::Json};

#[derive(Debug, Clone, PartialEq, Eq, FromRow, Builder, Getters, Setters)]
#[setters(prefix = "set_")]
#[setters(into)]
pub struct BusinessUnit {
    #[setters(skip)]
    id: Id,
    #[setters(skip)]
    code: Code,
    name: Name,

    vars: Json<Vars>,

    #[setters(skip)]
    created_at: Timestamp,
    updated_at: Timestamp,
}

impl Entity for BusinessUnit {
    fn id(&self) -> &Id {
        &self.id
    }
}
