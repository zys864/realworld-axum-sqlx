use tower::ServiceBuilder;
use tower_http::ServiceBuilderExt;
pub mod auth;
pub mod db;
pub mod error;
pub mod handler;
pub mod response_type;
pub mod utils;

pub type DbPool = sqlx::PgPool;
#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();
    let db = db::db().await;

    let middleware = ServiceBuilder::new().trace_for_http();
    let app = handler::app_router(db).layer(middleware);

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
