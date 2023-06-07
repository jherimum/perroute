use omni_commons::types::code::Code;
use omni_storage::models::channel::Channel;
use sqlx::PgPool;
use std::str::FromStr;

#[sqlx::test]
async fn test_channel_save(pool: PgPool) {
    let channel = Channel::new(Code::from_str("CODE").unwrap(), "Channel Wine");
    let channel = channel.save(&pool).await.unwrap();

    assert_eq!(
        Channel::find_by_id(&pool, &channel.id)
            .await
            .unwrap()
            .unwrap(),
        channel
    );
}

#[sqlx::test]
async fn test_channel_find_by_id(pool: PgPool) {
    let channel = Channel::find_by_id(&pool, &uuid::Uuid::new_v4())
        .await
        .unwrap();
    assert!(channel.is_none());
}
