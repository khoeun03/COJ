use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

pub async fn init_pool(database_url: &str) -> PgPool {
    PgPoolOptions::new()
        .max_connections(16)
        .connect(database_url)
        .await
        .expect("Failed to connect to database")
}
