use axum::{
    extract::{MatchedPath, Request},
    middleware::Next,
    response::IntoResponse,
    routing::get,
    Router,
};
use metrics_exporter_prometheus::{Matcher, PrometheusBuilder, PrometheusHandle};
use std::{future::ready, time::Instant};

pub async fn start_metrics_server() {
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
        10.0,
        50.0,
        100.0,
        150.0,
        200.0,
        250.0,
        300.0,
        400.0,
        500.0,
        600.0,
        700.0,
        800.0,
        1_000.0,
        1_500.0,
        2_000.0,
        2_500.0,
        3_000.0,
        4_000.0,
        5_000.0,
        10_000.0,
        20_000.0,
        40_000.0,
        60_000.0,
        80_000.0,
        100_000.0,
        300_000.0,
        500_000.0,
        1_000_000.0,
        3_000_000.0,
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

pub async fn track_metrics(req: Request, next: Next) -> impl IntoResponse {
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

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn setup_tracing() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}
