use std::sync::Arc;

use sqlx::PgPool;

pub type ArcPool = Arc<PgPool>;
