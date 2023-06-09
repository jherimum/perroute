use crate::connection_manager::{ConnectionManager, ConnectionPool};
use perroute_commons::configuration::settings::{DatabaseSettings, Settings};
use sqlx::{postgres::PgPoolOptions, Executor, PgConnection};
use std::fmt::Debug;
use tap::TapFallible;
use tokio::runtime::Handle;

#[derive(Debug)]
pub struct TestContext {
    pub pool: ConnectionPool,
    pub master_conn: PgConnection,
    pub db_name: String,
}

impl TestContext {
    pub async fn from_env() -> Self {
        let settings = Settings::load().expect("Failed to load settings");
        Self::new(&settings.database).await
    }

    pub async fn new(settings: &DatabaseSettings) -> Self {
        let db_name = uuid::Uuid::new_v4().to_string();
        let mut master = ConnectionManager::new_connection(settings).await.unwrap();
        Self::create_database(&mut master, db_name.to_owned()).await;
        let pool = Self::build_pool(settings, db_name.to_owned()).await;
        ConnectionManager::migrate(&pool).await.unwrap();
        Self {
            db_name,
            pool,
            master_conn: master,
        }
    }

    async fn build_pool(settings: &DatabaseSettings, database_name: String) -> ConnectionPool {
        let conn_options = ConnectionManager::connection_options(settings).database(&database_name);

        PgPoolOptions::new()
            .max_connections(2)
            .after_release(|_conn, _| Box::pin(async move { Ok(false) }))
            .connect_with(conn_options.clone())
            .await
            .tap_err(|e| {
                tracing::error!(
                    "Failed to initialize database with options:{:?}. {e}",
                    &conn_options
                )
            })
            .unwrap()
    }

    async fn create_database(conn: &mut PgConnection, db_name: String) {
        conn.execute(format!(r#"CREATE DATABASE "{}";"#, db_name).as_str())
            .await
            .unwrap();
    }

    async fn drop_database(&mut self) {
        self.master_conn
            .execute(format!(r#"DROP DATABASE "{}";"#, &self.db_name).as_str())
            .await
            .unwrap();
    }
}

impl Drop for TestContext {
    fn drop(&mut self) {
        tokio::task::block_in_place(move || {
            Handle::current().block_on(async {
                self.drop_database().await;
            });
        });
    }
}
