use chrono::{NaiveDateTime, Utc};
use derive_builder::Builder;
use derive_getters::Getters;
use derive_setters::Setters;
use perroute_commons::types::id::Id;
use perroute_messaging::events::{Event, EventType};
use sqlx::{FromRow, PgExecutor};

impl From<&DbEvent> for Event {
    fn from(value: &DbEvent) -> Self {
        Self::new(value.entity_id, value.event_type.clone())
    }
}

impl From<&Event> for DbEvent {
    fn from(value: &Event) -> Self {
        DbEventBuilder::default()
            .id(Id::new())
            .entity_id(*value.entity_id())
            .event_type(value.ty().clone())
            .created_at(Utc::now().naive_utc())
            .scheduled_to(Utc::now().naive_utc())
            .build()
            .unwrap()
    }
}

#[derive(Debug, FromRow, PartialEq, Eq, Clone, Getters, Setters, Builder)]
#[builder(setter(into))]
#[setters(prefix = "set_")]
#[setters(into)]
pub struct DbEvent {
    id: Id,
    entity_id: Id,
    event_type: EventType,
    created_at: NaiveDateTime,
    scheduled_to: NaiveDateTime,
    #[builder(default)]
    consumed_at: Option<NaiveDateTime>,
}

impl DbEvent {
    pub async fn fetch_unconsumed<'e, E: PgExecutor<'e>>(
        exec: E,
        limit: i64,
    ) -> Result<Vec<DbEvent>, sqlx::Error> {
        sqlx::query_as(
            r#"
                    select * 
                    from events 
                    where consumed_at is null
                    order by scheduled_to asc
                    limit $1
                "#,
        )
        .bind(limit)
        .fetch_all(exec)
        .await
    }

    pub async fn save<'e, E: PgExecutor<'e>>(&self, exec: E) -> Result<DbEvent, sqlx::Error> {
        sqlx::query_as(
            r#"
            INSERT INTO events (id, entity_id, event_type, created_at, scheduled_to)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *
            "#,
        )
        .bind(self.id)
        .bind(self.entity_id)
        .bind(self.event_type.clone())
        .bind(self.created_at)
        .bind(self.scheduled_to)
        .fetch_one(exec)
        .await
    }

    pub async fn update<'e, E: PgExecutor<'e>>(&self, exec: E) -> Result<DbEvent, sqlx::Error> {
        sqlx::query_as(
            r#"
            UPDATE events
            SET consumed_at = $1
            WHERE id = $2
            RETURNING *
            "#,
        )
        .bind(self.consumed_at)
        .bind(self.id)
        .fetch_one(exec)
        .await
    }
}
