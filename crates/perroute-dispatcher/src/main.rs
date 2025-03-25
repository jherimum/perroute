use perroute_commons::configuration::settings::Settings;
use perroute_connectors::plugin_repository;
use perroute_dispatcher::{dispatcher::create_dispatcher, pooling::SqsPooling};
use perroute_storage::repository::pgrepository::PgRepository;
use perroute_template::{
    render::handlebars::HandlebarsPlugin,
    repository::aws_s3::AwsS3TemplateRepository,
};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv()?;
    let settings = Settings::load()?;
    let aws_settings = settings.aws.as_ref().unwrap();
    let sdk_config = aws_config::load_from_env().await;
    let sqs_client = aws_sdk_sqs::Client::new(&sdk_config);

    let dispatcher = create_dispatcher(
        PgRepository,
        HandlebarsPlugin::new(),
        AwsS3TemplateRepository::new(&sdk_config, "bucket"),
        plugin_repository(),
    );

    SqsPooling::new(
        dispatcher,
        sqs_client,
        &aws_settings.digest_queue_url,
        Duration::from_secs(1),
        10,
    )
    .run()
    .await?;

    Ok(())
}
