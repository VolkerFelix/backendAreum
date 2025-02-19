use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

use crate::config::get_database_url;

pub async fn init_db() -> PgPool {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&get_database_url())
        .await
        .expect("Failed to connect to database")
}