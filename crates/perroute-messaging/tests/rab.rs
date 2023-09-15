use lapin::{options::BasicPublishOptions, BasicProperties};
use perroute_messaging::rabbitmq::connection::RabbitmqConnection;
use std::time::Duration;

#[tracing_test::traced_test]
#[tokio::test]
async fn conn() -> Result<(), anyhow::Error> {
    let conn = RabbitmqConnection::connect("amqp://admin:admin@localhost:5672").await?;

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
