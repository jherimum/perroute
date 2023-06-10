use perroute_commons::types::id::Id;

pub enum Actor {
    User(Id),
    System,
    Service(String), //api key
}
