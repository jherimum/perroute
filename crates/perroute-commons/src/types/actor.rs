use super::id::Id;

pub enum Actor {
    User(Id),
    System,
    Service(Id),
}
