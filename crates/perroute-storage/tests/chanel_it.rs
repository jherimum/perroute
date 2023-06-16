use perroute_commons::{
    code, new_id,
    types::{code::Code, id::Id},
};
use perroute_storage::models::channel::Channel;
use sqlx::PgPool;
use std::str::FromStr;

#[sqlx::test]
async fn test_channel_find_by_id(pool: PgPool) {
    let channel = Channel::find_by_id(&pool, new_id!()).await.unwrap();
    assert!(channel.is_none());

    let channel = Channel::new(new_id!(), code!("CODE"), "channel name")
        .save(&pool)
        .await
        .unwrap();

    assert_eq!(
        Channel::find_by_id(&pool, *channel.id()).await.unwrap(),
        Some(channel)
    );
}

#[sqlx::test]
async fn test_channel_find_by_code(pool: PgPool) {
    let code = code!("CODE");

    let channel = Channel::find_by_code(&pool, code.clone()).await.unwrap();
    assert!(channel.is_none());

    let channel = Channel::new(new_id!(), code.clone(), "channel name")
        .save(&pool)
        .await
        .unwrap();

    assert_eq!(
        Channel::find_by_code(&pool, code).await.unwrap(),
        Some(channel)
    );
}

#[sqlx::test]
async fn test_channel_save(pool: PgPool) {
    let channel_id = new_id!();
    let channel = Channel::new(channel_id, Code::from_str("CODE").unwrap(), "Channel Wine")
        .save(&pool)
        .await
        .unwrap();

    assert_eq!(
        Channel::find_by_id(&pool, channel_id).await.unwrap(),
        Some(channel)
    );
}

#[sqlx::test]
async fn test_channel_update(pool: PgPool) {
    let channel_id = new_id!();
    let mut channel = Channel::new(channel_id, code!("CODE"), "Channel Name")
        .save(&pool)
        .await
        .unwrap();

    channel.with_name("New Channel Name".to_owned());

    let channel = channel.update(&pool).await.unwrap();

    assert_eq!(
        Channel::find_by_id(&pool, channel_id).await.unwrap(),
        Some(channel)
    );
}

#[sqlx::test]
async fn test_exists_by_code(pool: PgPool) {
    let code = code!("CODE");
    assert!(!Channel::exists_by_code(&pool, code.clone()).await.unwrap());

    Channel::new(new_id!(), code.clone(), "Channel Wine")
        .save(&pool)
        .await
        .unwrap();

    assert!(Channel::exists_by_code(&pool, code).await.unwrap());
}

#[sqlx::test]
async fn test_channel_delete(pool: PgPool) {
    let channel_id = new_id!();

    let channel = Channel::new(channel_id, code!("CODE"), "Channel Wine")
        .save(&pool)
        .await
        .unwrap();

    assert!(Channel::find_by_id(&pool, channel_id)
        .await
        .unwrap()
        .is_some());

    let deleted = channel.delete(&pool).await.unwrap();
    assert!(deleted);

    assert!(Channel::find_by_id(&pool, channel_id)
        .await
        .unwrap()
        .is_none());
}
