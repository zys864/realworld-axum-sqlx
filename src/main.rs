use tower::ServiceBuilder;
use tower_http::ServiceBuilderExt;
pub mod auth;
pub mod db;
pub mod error;
pub mod handler;
pub mod jwt;
pub mod response_type;
#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();
    let db = db::db().await;

    let middleware = ServiceBuilder::new().trace_for_http().add_extension(db);
    let app = handler::app().layer(middleware);

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
