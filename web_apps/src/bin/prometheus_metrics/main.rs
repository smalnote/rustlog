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
use std::time::Duration;
use tokio::{signal, time::sleep};
use tracing::debug;

#[tokio::main]
async fn main() {
    observ::setup_tracing();
    let (_main_server, _metrics_server) = tokio::join!(start_main_server(), start_metrics_server());
}

async fn start_main_server() -> Result<(), std::io::Error> {
    let app = main_app();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
}

fn main_app() -> Router {
    Router::new()
        .route("/devices", get(routes::devices))
        .route_layer(middleware::from_fn(observ::track_metrics))
}

pub async fn start_metrics_server() -> Result<(), std::io::Error> {
    let app = observ::metrics_app();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await?;
    tracing::debug!(
        "metrics server listening on {}",
        listener.local_addr().unwrap()
    );
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
}

// Graceful shutdown signal handling
async fn shutdown_signal() {
    // Wait for Ctrl+C (SIGINT) signal
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
        println!("Received Ctrl+C signal, starting shutdown...");
    };

    // Optionally, you could listen for other signals like SIGTERM
    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install SIGTERM handler")
            .recv()
            .await;
        println!("Received SIGTERM signal, starting shutdown...");
    };

    #[cfg(unix)]
    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    #[cfg(not(unix))]
    ctrl_c.await;

    // Optionally, add some delay or cleanup before shutdown
    println!("Shutting down gracefully in 5 seconds...");
    sleep(Duration::from_secs(5)).await;
}
