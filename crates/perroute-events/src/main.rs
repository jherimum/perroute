use anyhow::Result;
use perroute_commons::{configuration::settings::Settings, tracing::init_tracing};
use perroute_messaging::{
    connection::{Config, RecoverableConnection},
    consumer::Consumer,
    events::Event,
    producer::Producer,
};
use perroute_storage::{connection_manager::ConnectionManager, models::db_event::DbEvent};
use sqlx::PgPool;
use std::time::Duration;
use tap::TapFallible;

const POOLING_START_DELAY: u64 = 2;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenv::dotenv().ok();
    init_tracing();
    let settings = Settings::load().tap_err(|e| tracing::error!("Failed to load settings: {e}"))?;
    let pool = ConnectionManager::build_pool(&settings.database)
        .await
        .tap_err(|e| tracing::error!("Failed to build pool: {e}"))?;
    let conn = RecoverableConnection::connect(Config {
        uri: settings.rabbitmq.unwrap().uri,
        time_out: Duration::from_secs(20),
        retry_delay: Duration::from_secs(1),
    })
    .await?;

    tracing::info!("Pooling will start in {}", POOLING_START_DELAY);
    tokio::time::sleep(Duration::from_secs(POOLING_START_DELAY)).await;

    let pooling_conn = conn.clone();

    let pooling = tokio::spawn(async move {
        tracing::info!("Starting pooling events");
        let producer = create_producer(&pooling_conn).await;
        loop {
            let _ = event_pooling(&pool, &producer)
                .await
                .tap_err(|e| tracing::error!("Failed to pool events: {e}"));
            tokio::time::sleep(Duration::from_secs(3)).await;
        }
    });

    Consumer {
        connection: &conn,
        tag: "read",
        queue: "read_events",
        exchange: "perroute.events",
        routing_key: "ChannelCreated",
        threads: 4,
        function: |e| tracing::info!("Event consumed: {e:?}"),
    }
    .start()
    .await;

    Ok(pooling
        .await
        .tap_err(|e| tracing::error!("Failed to join pooling: {e}"))?)
}

#[async_recursion::async_recursion]
async fn create_producer<'c>(conn: &'c RecoverableConnection) -> Producer<'c> {
    let producer = Producer::new(conn, "perroute.events", true)
        .await
        .tap_err(|e| tracing::error!("Failed to create producer: {e}"));

    if producer.is_err() {
        return create_producer(conn).await;
    }

    producer.unwrap()
}

async fn event_pooling<'c>(pool: &PgPool, producer: &Producer<'c>) -> Result<(), anyhow::Error> {
    let events = DbEvent::fetch_unconsumed(pool, 10)
        .await
        .tap_err(|e| tracing::error!("Failed to poll events form database: {e}"))?;

    tracing::debug!("Polled {} events from database", events.len());

    for db_event in events {
        let event_id = *db_event.id();
        let event: Event = Event::from(&db_event);

        let sent_result = producer
            .send(&event, Some(event.ty().to_string().as_str()))
            .await
            .tap_ok(|_| tracing::debug!("Event [{}] sent", event_id))
            .tap_err(|e| tracing::error!("Failed to send event [{}]: {}", event_id, e));

        if sent_result.is_ok() {
            let _ = db_event
                //.set_consumed_at(Utc::now().naive_utc())
                .update(pool)
                .await
                .tap_err(|e| {
                    tracing::error!("Failed to set event [{}] as consumed: {}", event_id, e)
                })
                .tap_ok(|_| tracing::info!("Event [{}] consumed", event_id));
        }
    }

    Ok(())
}
