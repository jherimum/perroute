use crate::types::id::Id;

pub trait Entity {
    fn id(&self) -> &Id;

    fn same_entity(&self, other: &Self) -> bool {
        self.id() == other.id()
    }

    fn ids(entities: &[Self]) -> Vec<&Id>
    where
        Self: Sized,
    {
        entities.iter().map(|e| e.id()).collect::<Vec<_>>()
    }
}
