use std::time::Duration;

use perroute_commons::{
    configuration::settings::Settings, template::handlebars::HandlebarsTemplateRender,
};
use perroute_digester::pooling::SqsPooling;
use perroute_storage::create_repository;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv()?;
    let settings = Settings::load()?;
    let aws_settings = settings.aws.unwrap();
    let sdk_config = aws_config::load_from_env().await;
    let sqs_client = aws_sdk_sqs::Client::new(&sdk_config);
    let s3_client = aws_sdk_s3::Client::new(&sdk_config);

    let repository = create_repository(&settings.database.unwrap()).await?;

    let template_render = template_render();

    SqsPooling::new(
        repository,
        template_render,
        sqs_client,
        s3_client,
        &aws_settings.digest_queue_url,
        Duration::from_secs(1),
        10,
        "perroute-digester",
        perroute_connectors::repository(),
    )
    .run()
    .await?;

    Ok(())
}

fn template_render<'tr>() -> HandlebarsTemplateRender<'tr> {
    todo!("Implement template render")
}
