// SPDX-FileCopyrightText: 2022 Foundation Devices Inc. <hello@foundationdevices.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Common HTTP functionality, applied to all services using `servus`.
//!
//! This module has one public function `serve`.

pub mod metrics;

use axum::{extract::Extension, http::StatusCode, middleware, routing, Router, Server};
use std::net::SocketAddr;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing::{error, Level};

/// Entrypoint for serving HTTP requests.
///
/// Initializes and starts the application `axum::Server`, and optionally another for metrics
/// and healthcheck output. The separate server for metrics prevents incoming connections from accessing
/// that route, keeping it internal to the deployed environment. (For instance, your primary
/// ingress configuration would point to the HTTP listen address, whereas some metrics scraping
/// process would point at the metrics listen address, keeping that data private from the outside
/// world.) The metrics server has two routes, `GET /metrics` and `GET /health`.
///
/// The given `axum::Router` parameter is used in the primary application server instance, and is
/// layered with metrics middleware. See the `servus::http::metrics` module for the default
/// metrics generated.
///
/// It also takes an optional application state. If the value provided is anything other than the
/// unit type `()`, the value is given to the application HTTP router as a `tower::Layer`, wrapped in
/// an `axum::extract::Extension`, so it is available to HTTP route handlers that need it.
///
/// Both the application server and metrics server will respond to a CTRL-C shutdown signal and
/// terminate gracefully.
pub async fn serve<S>(
    http_address: SocketAddr,
    metrics_address: Option<SocketAddr>,
    router: Router,
    state: S,
) where
    S: Send + Sync + Clone + 'static,
{
    // create primary application router and server
    // applying handler state if we have it, and default metrics/tracing middleware
    let r = router
        .layer(Extension(state))
        .route_layer(middleware::from_fn(metrics::middleware)) // only record matched routes
        .layer(
            TraceLayer::new_for_http().make_span_with(
                DefaultMakeSpan::new()
                    .level(Level::TRACE)
                    .include_headers(true),
            ),
        );

    let app = Server::bind(&http_address)
        .serve(r.into_make_service())
        .with_graceful_shutdown(shutdown_signal());

    if let Some(metrics_address) = metrics_address {
        // create metrics router and server, also used for healthcheck
        let r = Router::new()
            .route("/metrics", routing::get(metrics::handler))
            .route("/health", routing::get(health));

        let metrics = Server::bind(&metrics_address)
            .serve(r.into_make_service())
            .with_graceful_shutdown(shutdown_signal());

        // spawn each server instance (so they can be scheduled on separate threads as necessary)
        // and wait on their join handles, returning early if one reports an error
        let app = tokio::spawn(app);
        let metrics = tokio::spawn(metrics);

        if let Err(e) = tokio::try_join!(app, metrics) {
            error!("server error = {}", e);
        }
    } else {
        // no metrics address given, only spawn single application instance
        if let Err(e) = app.await {
            error!("server error = {}", e);
        }
    }
}

async fn health() -> StatusCode {
    StatusCode::OK
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c().await.expect("setup ctrl-c signal");
}
