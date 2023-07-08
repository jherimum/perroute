use perroute_commons::{code, new_id};
use perroute_storage::models::channel::{Channel, ChannelBuilder};
use sqlx::PgPool;
use std::str::FromStr;

#[sqlx::test]
async fn test_channel_find_by_id(pool: PgPool) {
    let channel = Channel::find_by_id(&pool, new_id!()).await.unwrap();
    assert!(channel.is_none());

    let channel = ChannelBuilder::default()
        .id(new_id!())
        .code(code!("CODE"))
        .name("channel name".to_owned())
        .build()
        .unwrap()
        .save(&pool)
        .await
        .expect("failed to save channel");

    assert_eq!(
        Channel::find_by_id(&pool, *channel.id()).await.unwrap(),
        Some(channel)
    );
}

#[sqlx::test]
async fn test_channel_find_by_code(pool: PgPool) {
    let code = code!("CODE");

    let channel = Channel::find_by_code(&pool, &code).await.unwrap();
    assert!(channel.is_none());

    let channel = ChannelBuilder::default()
        .id(new_id!())
        .code(code.clone())
        .name("channel name".to_owned())
        .build()
        .unwrap()
        .save(&pool)
        .await
        .unwrap();

    assert_eq!(
        Channel::find_by_code(&pool, &code).await.unwrap(),
        Some(channel)
    );
}

#[sqlx::test]
async fn test_channel_save(pool: PgPool) {
    let channel_id = new_id!();
    let channel = ChannelBuilder::default()
        .id(channel_id)
        .code(code!("CODE"))
        .name("channel name".to_owned())
        .build()
        .unwrap()
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
    let channel = ChannelBuilder::default()
        .id(channel_id)
        .code(code!("CODE"))
        .name("channel name".to_owned())
        .build()
        .unwrap()
        .save(&pool)
        .await
        .unwrap();

    let channel = channel
        .set_name("New Channel Name".to_owned())
        .update(&pool)
        .await
        .unwrap();

    assert_eq!(
        Channel::find_by_id(&pool, channel_id).await.unwrap(),
        Some(channel)
    );
}

#[sqlx::test]
async fn test_exists_by_code(pool: PgPool) {
    let code = code!("CODE");
    assert!(!Channel::exists_by_code(&pool, &code).await.unwrap());

    ChannelBuilder::default()
        .id(new_id!())
        .code(code.clone())
        .name("channel name".to_owned())
        .build()
        .unwrap()
        .save(&pool)
        .await
        .unwrap();

    assert!(Channel::exists_by_code(&pool, &code).await.unwrap());
}

#[sqlx::test]
async fn test_channel_delete(pool: PgPool) {
    let channel_id = new_id!();

    let channel = ChannelBuilder::default()
        .id(channel_id)
        .code(code!("CODE"))
        .name("channel name".to_owned())
        .build()
        .unwrap()
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

#[sqlx::test]
async fn test_query(pool: PgPool) {
    let channels = Channel::query(&pool)
        .await
        .expect("failed to query channels");

    assert!(channels.is_empty());

    let channel_1 = ChannelBuilder::default()
        .id(new_id!())
        .code(code!("CODE"))
        .name("channel name".to_owned())
        .build()
        .unwrap()
        .save(&pool)
        .await
        .expect("failed to save channel");

    let channel_2 = ChannelBuilder::default()
        .id(new_id!())
        .code(code!("CODE1"))
        .name("channel name".to_owned())
        .build()
        .unwrap()
        .save(&pool)
        .await
        .expect("failed to save channel");

    let channels = Channel::query(&pool)
        .await
        .expect("failed to query channels");

    assert!(channels.contains(&channel_1));
    assert!(channels.contains(&channel_2));
}
