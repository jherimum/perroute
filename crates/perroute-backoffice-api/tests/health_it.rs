mod common;

use crate::common::TestApp;
use sqlx::PgPool;

#[sqlx::test]
async fn test_health(pool: PgPool) {
    let app = TestApp::start(pool).await;
    let resp = reqwest::get(app.path(["health"])).await.unwrap();
    assert_eq!(resp.status(), 200, "health should response 200");
}
