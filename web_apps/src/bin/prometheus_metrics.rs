//! Binary crate prometheus_metrics is a example of web application build on the
//! top of tokio, axum, metrics and metrics-exporter-prometheus.
//!
//! Run with
//!
//! ```bash
//! cargo run -p web_apps --bin prometheus_metrics
//! ```

use std::{
    future::ready,
    time::{Duration, Instant},
};

use axum::{
    extract::{MatchedPath, Request},
    middleware::{self, Next},
    response::IntoResponse,
    routing::get,
    Router,
};
use metrics_exporter_prometheus::{Matcher, PrometheusBuilder, PrometheusHandle};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    let (_main_server, _metrics_server) = tokio::join!(start_main_server(), start_metrics_server());
}

async fn start_main_server() {
    let app = main_app();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("listen on tcp address failed");
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

fn main_app() -> Router {
    Router::new()
        .route("/fast", get(|| async {}))
        .route(
            "/slow",
            get(|| async {
                tokio::time::sleep(Duration::from_secs(1)).await;
            }),
        )
        .route_layer(middleware::from_fn(track_metrics))
}

async fn track_metrics(req: Request, next: Next) -> impl IntoResponse {
    let start = Instant::now();
    let path = if let Some(matched_path) = req.extensions().get::<MatchedPath>() {
        matched_path.as_str().to_owned()
    } else {
        req.uri().path().to_owned()
    };
    let method = req.method().clone().to_string();

    let response = next.run(req).await;

    let latency = start.elapsed().as_micros();
    let status = response.status().as_u16().to_string();

    let labels = [("method", method), ("path", path), ("status", status)];

    metrics::counter!("http_request_total", &labels).increment(1);
    metrics::histogram!("http_request_duration_microseconds", &labels).record(latency as f64);

    response
}

async fn start_metrics_server() {
    let app = metrics_app();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .expect("listen on metrics port failed");
    tracing::debug!(
        "metrics server listening on {}",
        listener.local_addr().unwrap()
    );
    axum::serve(listener, app).await.unwrap();
}

fn metrics_app() -> Router {
    let recorder_handle = setup_metrics_recorder();
    Router::new().route("/metrics", get(move || ready(recorder_handle.render())))
}

fn setup_metrics_recorder() -> PrometheusHandle {
    const EXPONENTIAL_MILLISECONDS: &[f64] = &[
        1.0,
        50.0,
        100.0,
        500.0,
        1_000.0,
        5_000.0,
        10_000.0,
        50_000.0,
        100_000.0,
        500_000.0,
        1_000_000.0,
        5_000_000.0,
        10_000_000.0,
    ];

    PrometheusBuilder::new()
        .set_buckets_for_metric(
            Matcher::Full("http_request_duration_microseconds".to_string()),
            EXPONENTIAL_MILLISECONDS,
        )
        .unwrap()
        .install_recorder()
        .unwrap()
}
