use sqlx::PgPool;

pub async fn db() -> PgPool {
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL env not be setted");
    PgPool::connect(&database_url).await.unwrap()
}
