use perroute_commons::types::code::Code;
use perroute_storage::models::channel::Channel;
use sqlx::PgPool;
use std::str::FromStr;
use uuid::uuid;

async fn test_channel_update(pool: PgPool) {
    let mut channel = Channel::find_by_id(&pool, &uuid!("00000000-0000-0000-0000-000000000000"))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(channel.name, "Channel Wine");

    channel.name = "Name Changed".to_owned();

    let channel = channel.update(&pool).await.unwrap();

    let channel = Channel::find_by_id(&pool, &uuid!("00000000-0000-0000-0000-000000000000"))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(channel.name, "Channel Wine");
}

#[sqlx::test(fixtures("channels"))]
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

#[sqlx::test(fixtures("channels"))]
async fn test_exists_by_code(pool: PgPool) {
    assert!(
        Channel::exists_by_code(&pool, &Code::from_str("WINE_B2C").unwrap())
            .await
            .unwrap()
    );

    assert!(
        !Channel::exists_by_code(&pool, &Code::from_str("UNKNOW").unwrap())
            .await
            .unwrap()
    );
}

#[sqlx::test(fixtures("channels"))]
async fn test_channel_find_by_id(pool: PgPool) {
    let channel = Channel::find_by_id(&pool, &uuid!("00000000-0000-0000-0000-000000000000"))
        .await
        .unwrap();
    assert!(channel.is_some());

    let channel = Channel::find_by_id(&pool, &uuid!("00000000-0000-1111-0000-000000000000"))
        .await
        .unwrap();
    assert!(channel.is_none());
}

#[sqlx::test(fixtures("channels"))]
async fn test_channel_find_by_code(pool: PgPool) {
    let channel = Channel::find_by_code(&pool, &Code::from_str("WINE_B2C").unwrap())
        .await
        .unwrap();
    assert!(channel.is_some());

    let channel = Channel::find_by_code(&pool, &Code::from_str("CODE2").unwrap())
        .await
        .unwrap();
    assert!(channel.is_none());
}

#[sqlx::test(fixtures("channels"))]
async fn test_channel_delete(pool: PgPool) {
    let persisted_channel_uuid = &uuid!("00000000-0000-0000-0000-000000000000");
    let channel = Channel::find_by_id(&pool, persisted_channel_uuid)
        .await
        .unwrap();
    assert!(channel.is_some());

    let channel = Channel::find_by_id(&pool, persisted_channel_uuid)
        .await
        .unwrap()
        .unwrap();
    assert!(channel.delete(&pool).await.unwrap());

    let channel = Channel::find_by_id(&pool, persisted_channel_uuid)
        .await
        .unwrap();
    assert!(channel.is_none());
}
