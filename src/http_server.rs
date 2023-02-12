use axum::body::Body;
use axum::body::Bytes;
use axum::headers::HeaderMap;
use http::Request;

use std::time::Duration;
use tower::ServiceBuilder;
use tower_http::ServiceBuilderExt;

use tower_http::{classify::ServerErrorsFailureClass, trace::TraceLayer};
use tracing::Span;

use crate::metrics_server::track_metrics;

pub async fn http_server() {
    let db = crate::db::db().await;

    let middleware = ServiceBuilder::new()
        // .timeout(std::time::Duration::from_secs(10))
        .compression()
        .trim_trailing_slash()
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|_request: &Request<Body>| {
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
                    (),
                )
                .on_body_chunk(|chunk: &Bytes, _latency: Duration, _span: &Span| {
                    tracing::debug!("sending {} bytes", chunk.len())
                })
                .on_eos(
                    |_trailers: Option<&HeaderMap>,
                     stream_duration: Duration,
                     _span: &Span| {
                        tracing::debug!("stream closed after {:?}", stream_duration)
                    },
                )
                .on_failure(
                    |error: ServerErrorsFailureClass,
                     _latency: Duration,
                     _span: &Span| {
                        tracing::error!(msg = "something went wrong", ?error)
                    },
                ),
        );
    let app = crate::handler::app_router(db)
        .route_layer(axum::middleware::from_fn(track_metrics))
        .layer(middleware);
    tracing::info!("listening host 127.0.0.1:3000");
    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .with_graceful_shutdown(crate::utils::graceful_shutdown::shutdown_signal(
            "http_server",
        ))
        .await
        .unwrap();
}
