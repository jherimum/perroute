use aws_config::Region;
use aws_sdk_sqs::config::Credentials;
use perroute_commons::configuration::settings::Settings;

use std::{borrow::Cow, error::Error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;
    let settings = Settings::load()?;
    let aws_settings = settings.aws.unwrap();
    let sdk_config = aws_config::load_from_env().await;
    let sqs_client = aws_sdk_sqs::Client::new(&sdk_config);

    dbg!(sdk_config.region());
    dbg!(&aws_settings.dispatch_queue_url);

    let messages = sqs_client
        .receive_message()
        .queue_url(&aws_settings.digest_queue_url)
        .max_number_of_messages(10)
        .send()
        .await;

    match messages {
        Ok(output) => {
            if let Some(messages) = output.messages {
                for message in messages {
                    println!("Message: {:?}", message);
                }
            }
        }
        Err(error) => {
            eprintln!("Error: {:?}", error);
        }
    }

    Ok(())
}
