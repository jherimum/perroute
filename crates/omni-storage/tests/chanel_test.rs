use omni_commons::types::code::Code;
use omni_storage::{models::channel::Channel, utils::TestContext};
use std::str::FromStr;

#[tokio::test(flavor = "multi_thread")]
async fn test_channel() {
    let context = TestContext::from_env().await;
    Channel::new(Code::from_str("CODE").unwrap(), "Channel Wine")
        .save(&context.pool)
        .await
        .unwrap();
}
