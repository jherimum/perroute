use perroute_commons::types::{code::Code, id::Id};
use perroute_storage::models::channel::Channel;
use sqlx::PgPool;
use std::str::FromStr;

#[sqlx::test]
async fn test_channel_find_by_id(pool: PgPool) {
    let channel = Channel::find_by_id(&pool, &Id::new()).await.unwrap();
    assert!(channel.is_none());

    let channel = Channel::new(&Code::from_str("CODE").unwrap(), "channel name")
        .save(&pool)
        .await
        .unwrap();

    assert_eq!(
        Channel::find_by_id(&pool, &channel.id).await.unwrap(),
        Some(channel)
    );
}

#[sqlx::test]
async fn test_channel_find_by_code(pool: PgPool) {
    let code = Code::from_str("CODE").unwrap();
    let channel = Channel::find_by_code(&pool, &code).await.unwrap();
    assert!(channel.is_none());

    let channel = Channel::new(&code, "channel name")
        .save(&pool)
        .await
        .unwrap();

    assert_eq!(
        Channel::find_by_code(&pool, &code).await.unwrap(),
        Some(channel)
    );
}

#[sqlx::test(fixtures("channels"))]
async fn test_channel_save(pool: PgPool) {
    let channel = Channel::new(&Code::from_str("CODE").unwrap(), "Channel Wine")
        .save(&pool)
        .await
        .unwrap();

    assert_eq!(
        Channel::find_by_id(&pool, &channel.id).await.unwrap(),
        Some(channel)
    );
}

#[sqlx::test(fixtures("channels"))]
async fn test_channel_update(pool: PgPool) {
    let mut channel = Channel::new(&Code::from_str("CODE").unwrap(), "Channel Wine")
        .save(&pool)
        .await
        .unwrap();

    channel.name = "Channel Wine".to_owned();

    let channel = channel.update(&pool).await.unwrap();

    assert_eq!(
        Channel::find_by_id(&pool, &channel.id).await.unwrap(),
        Some(channel)
    );
}

#[sqlx::test(fixtures("channels"))]
async fn test_exists_by_code(pool: PgPool) {
    let code = Code::from_str("CODE").unwrap();
    assert!(!Channel::exists_by_code(&pool, &code).await.unwrap());

    let channel = Channel::new(&code, "Channel Wine")
        .save(&pool)
        .await
        .unwrap();

    assert!(Channel::exists_by_code(&pool, &code).await.unwrap());
}

#[sqlx::test(fixtures("channels"))]
async fn test_channel_delete(pool: PgPool) {
    let code = Code::from_str("CODE").unwrap();

    let channel = Channel::new(&code, "Channel Wine")
        .save(&pool)
        .await
        .unwrap();

    assert!(Channel::find_by_code(&pool, &code).await.unwrap().is_some());

    let deleted = channel.delete(&pool).await.unwrap();
    assert!(deleted);

    assert!(Channel::find_by_code(&pool, &code).await.unwrap().is_none());
}
