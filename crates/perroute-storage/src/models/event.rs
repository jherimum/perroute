use perroute_commons::types::id::Id;
use time::OffsetDateTime;

pub struct Event {
    pub id: Id,
    pub object_id: Id,

    pub created_at: OffsetDateTime,
}
