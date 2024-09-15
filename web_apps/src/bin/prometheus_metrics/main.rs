//! Binary crate prometheus_metrics is a example of web application build on the
//! top of tokio, axum, metrics and metrics-exporter-prometheus.
//!
//! Run with
//!
//! ```bash
//! cargo run -p web_apps --bin prometheus_metrics
//! ```

mod device;
mod observability;
mod routes;

use observability as observ;

use axum::{middleware, routing::get, Router};
use tracing::debug;

#[tokio::main]
async fn main() {
    observ::setup_tracing();
    let (_main_server, _metrics_server) = tokio::join!(start_main_server(), start_metrics_server());
}

async fn start_main_server() {
    let app = main_app();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("listen on tcp address failed");
    debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

fn main_app() -> Router {
    Router::new()
        .route("/devices", get(routes::devices))
        .route_layer(middleware::from_fn(observ::track_metrics))
}

pub async fn start_metrics_server() {
    let app = observ::metrics_app();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001")
        .await
        .expect("listen on metrics port failed");
    tracing::debug!(
        "metrics server listening on {}",
        listener.local_addr().unwrap()
    );
    axum::serve(listener, app).await.unwrap();
}
