use perroute_commons::types::{id::Id, properties::Properties};

pub struct CreateRouteCommand {
    id: Id,
    channel_id: Id,
    schema_id: Id,
    properties: Properties,
}
