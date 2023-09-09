use lapin::{options::BasicPublishOptions, BasicProperties};
use perroute_messaging::rabbitmq::connection::{Config, RabbitmqConnection};
use std::time::Duration;

#[tracing_test::traced_test]
#[tokio::test]
async fn conn() -> Result<(), anyhow::Error> {
    let config = Config {
        uri: "amqp://admin:admin@localhost:5672".to_lowercase(),
        time_out: Duration::from_secs(20),
        retry_delay: Duration::from_secs(1),
    };
    let conn = RabbitmqConnection::connect(config).await?;

    loop {
        match conn
            .create_channel()
            .get()
            .await?
            .basic_publish(
                "amq.direct",
                "x",
                BasicPublishOptions::default(),
                "Teste".as_bytes(),
                BasicProperties::default(),
            )
            .await
        {
            Ok(_) => tracing::info!("Published"),
            Err(e) => tracing::error!("Error: {:?}", e),
        }
        tokio::time::sleep(Duration::from_secs(5)).await
    }
}
