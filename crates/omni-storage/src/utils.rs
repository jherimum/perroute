use crate::connection_manager::{ConnectionPool, MigrantionMode, OmniMessageConnectionManager};
use omni_commons::configuration::settings::{DatabaseSettings, Settings};
use sqlx::{Connection, Executor};
use tokio::runtime::Handle;

pub struct TestContext {
    pub settings: DatabaseSettings,
    pub db_name: String,
    pub pool: ConnectionPool,
}

impl TestContext {
    pub async fn from_env() -> Self {
        let settings = Settings::load().expect("Failed to load settings");
        Self::new(settings.database).await
    }

    pub async fn new(settings: DatabaseSettings) -> Self {
        let db_name = uuid::Uuid::new_v4().to_string();
        Self::create_database(&settings, &db_name).await;
        let pool = OmniMessageConnectionManager::new_pool(
            &settings,
            MigrantionMode::Run,
            Some(db_name.clone()),
        )
        .await
        .unwrap();
        Self {
            settings,
            db_name,
            pool,
        }
    }

    async fn create_database(settings: &DatabaseSettings, db_name: &str) {
        let mut master = OmniMessageConnectionManager::new_connection(settings, None)
            .await
            .unwrap();

        master
            .execute(format!(r#"CREATE DATABASE "{}";"#, &db_name).as_str())
            .await
            .unwrap();

        master.close().await.unwrap();
    }

    async fn drop_database(&self) {
        let mut conn = OmniMessageConnectionManager::new_connection(&self.settings, None)
            .await
            .unwrap();

        conn.execute(format!(r#"DROP DATABASE "{}";"#, &self.db_name).as_str())
            .await
            .unwrap();

        conn.close().await.unwrap();
    }
}

impl Drop for TestContext {
    fn drop(&mut self) {
        tokio::task::block_in_place(move || {
            Handle::current().block_on(async {
                self.pool.close().await;
                self.drop_database().await;
            })
        });
    }
}
