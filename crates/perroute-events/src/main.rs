use aws_config::SdkConfig;
use aws_sdk_sns::Client;
use perroute_commons::configuration::settings::Settings;
use perroute_events::Event;
use perroute_storage::{connection_manager::ConnectionManager, models::db_event::DbEvent};
use sqlx::PgPool;
use tokio_cron_scheduler::{Job, JobScheduler, JobSchedulerError};
#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let config = aws().await;
    let client = aws_sdk_sns::Client::new(&config);
    let settings = Settings::load().unwrap();
    let pool = ConnectionManager::build_pool(&settings.database)
        .await
        .unwrap();

    let sched = JobScheduler::new().await.unwrap();
    sched
        .add(event_pooling(pool, client).await.unwrap())
        .await
        .unwrap();

    sched.start().await.unwrap();
    tokio::time::sleep(core::time::Duration::from_secs(10)).await;
}

async fn event_pooling(pool: PgPool, aws: Client) -> Result<Job, JobSchedulerError> {
    Job::new_async("1/1 * * * * *", move |_, _| {
        let pool = pool.clone();
        let aws = aws.clone();
        Box::pin(async move {
            for db_event in DbEvent::all(&pool).await.unwrap() {
                let event: Event = db_event.try_into().unwrap();
                let json = serde_json::to_string(&event).unwrap();
                let x = aws.create_topic().name("events").send().await.unwrap();
                aws.publish()
                    .topic_arn("arn:aws:sns:us-east-1:720506629679:events")
                    .message(json)
                    .send()
                    .await
                    .unwrap();
                println!("{:?}", event);
            }
        })
    })
}

async fn aws() -> SdkConfig {
    aws_config::from_env().region("us-east-1").load().await
}
