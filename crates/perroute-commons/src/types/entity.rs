use crate::types::id::Id;

pub trait Entity {
    fn id(&self) -> &Id;

    fn same_entity(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}
