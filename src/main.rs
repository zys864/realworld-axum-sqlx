use http_server::http_server;
use metrics_server::metrics_server;
use utils::log_utils::log_init;

pub mod auth;
pub mod db;
pub mod error;
pub mod handler;
pub mod http_server;
pub mod metrics_server;
pub mod response_type;
pub mod utils;

pub type DbPool = sqlx::PgPool;
#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let _guard = log_init();
    // The `/metrics` endpoint should not be publicly available. If behind a reverse proxy, this
    // can be achieved by rejecting requests to `/metrics`. In this example, a second server is
    // started on another port to expose `/metrics`.
    let (_main_server, _metrics_server) = tokio::join!(http_server(), metrics_server());
}
