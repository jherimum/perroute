pub mod database;
pub mod models;
pub mod repository;

use perroute_commons::configuration::settings::DatabaseSettings;
use repository::PgRepository;
use repository::RepositoryResult;

pub async fn create_repository(settings: &DatabaseSettings) -> RepositoryResult<PgRepository> {
    PgRepository::from_settings(settings).await
}

#[macro_export]
macro_rules! fetch_all {
    ($conn:expr,$query:expr) => {
        match $conn {
            $crate::repository::Source::Pool(pool) => $query.fetch_all(pool).await,
            $crate::repository::Source::Tx(tx) => {
                let mut x = tx.lock().await;
                $query.fetch_all(x.as_mut()).await
            }
        }
    };
}

#[macro_export]
macro_rules! fetch_one {
    ($conn:expr,$query:expr) => {
        match $conn {
            $crate::repository::Source::Pool(pool) => $query.fetch_one(pool).await,
            $crate::repository::Source::Tx(tx) => {
                let mut x = tx.lock().await;
                $query.fetch_one(x.as_mut()).await
            }
        }
    };
}

#[macro_export]
macro_rules! fetch_optional {
    ($conn:expr,$query:expr) => {
        match $conn {
            $crate::repository::Source::Pool(pool) => $query.fetch_optional(pool).await,
            $crate::repository::Source::Tx(tx) => {
                let mut x = tx.lock().await;
                $query.fetch_optional(x.as_mut()).await
            }
        }
    };
}

#[macro_export]
macro_rules! execute {
    ($conn:expr,$query:expr) => {
        match $conn {
            $crate::repository::Source::Pool(pool) => $query.execute(pool).await,
            $crate::repository::Source::Tx(tx) => {
                let mut x = tx.lock().await;
                $query.execute(x.as_mut()).await
            }
        }
    };
}
