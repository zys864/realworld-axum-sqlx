use tower::ServiceBuilder;
use tower_http::ServiceBuilderExt;

pub mod db;
pub mod handler;
pub mod response_type;
pub mod error;
#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();
    let db = db::db().await;
    
    let middleware = ServiceBuilder::new()
    .add_extension(db);
    let app = handler::app()
    .layer(middleware);
    
    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
