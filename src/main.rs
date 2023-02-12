use axum::body::Body;
use axum::body::Bytes;
use axum::headers::HeaderMap;
use http::Request;
use http::Response;
use std::time::Duration;
use tower::ServiceBuilder;
use tower_http::ServiceBuilderExt;
use utils::log_utils::log_init;

use tower_http::{classify::ServerErrorsFailureClass, trace::TraceLayer};
use tracing::Span;
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
    let _guard = log_init();
    let db = db::db().await;

    let middleware = ServiceBuilder::new()
        // .timeout(std::time::Duration::from_secs(10))
        .compression()
        .trim_trailing_slash()
        .propagate_x_request_id()
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<Body>| {
                    tracing::info_span!("http-request")
                })
                .on_request(|request: &Request<Body>, _span: &Span| {
                    tracing::info!(
                        "started {} {:?} {}",
                        request.method(),
                        request.headers(),
                        request.uri().path()
                    )
                })
                .on_response(
                    // |response: &Response<Body>, latency: Duration, _span: &Span| {
                    //     tracing::debug!("response generated in {:?}", latency)
                    // },
                    ()
                )
                .on_body_chunk(|chunk: &Bytes, latency: Duration, _span: &Span| {
                    tracing::debug!("sending {} bytes", chunk.len())
                })
                .on_eos(
                    |trailers: Option<&HeaderMap>,
                     stream_duration: Duration,
                     _span: &Span| {
                        tracing::debug!("stream closed after {:?}", stream_duration)
                    },
                )
                .on_failure(
                    |error: ServerErrorsFailureClass, latency: Duration, _span: &Span| {
                        tracing::debug!("something went wrong")
                    },
                ),
        );
    let app = handler::app_router(db).layer(middleware);
    tracing::info!("listening host 127.0.0.1:3000");
    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}


