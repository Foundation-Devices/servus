pub mod metrics;

use axum::{extract::Extension, http::StatusCode, middleware, routing, Router, Server};
use std::net::SocketAddr;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing::{error, Level};

// Entrypoint for serving HTTP requests.
// Creates two instances of a `hyper::Server`, one for application routes, and another for metrics
// and healthcheck output. The separate server for metrics prevents incoming connections from accessing
// that route, keeping it internal to the deployed environment.
pub async fn serve<S>(addrs: (SocketAddr, SocketAddr), state: S, router: Router)
where
    S: Send + Sync + Clone + 'static,
{
    // create primary application router and server,
    // applying handler state, default middleware, and tracing
    let r = router
        .layer(Extension(state))
        .layer(middleware::from_fn(metrics::middleware))
        .layer(
            TraceLayer::new_for_http().make_span_with(
                DefaultMakeSpan::new()
                    .level(Level::TRACE)
                    .include_headers(true),
            ),
        );

    let app = Server::bind(&addrs.0)
        .serve(r.into_make_service())
        .with_graceful_shutdown(shutdown_signal());

    // create metrics router and server, also used for healthcheck
    let r = Router::new()
        .route("/metrics", routing::get(metrics::handler))
        .route("/health", routing::get(health));

    let metrics = Server::bind(&addrs.1)
        .serve(r.into_make_service())
        .with_graceful_shutdown(shutdown_signal());

    // spawn each server instance (so they can be scheduled on separate threads as necessary)
    // and wait on their join handles, returning early if one reports an error
    let app = tokio::spawn(app);
    let metrics = tokio::spawn(metrics);

    if let Err(e) = tokio::try_join!(app, metrics) {
        error!("server error = {}", e);
    }
}

async fn health() -> StatusCode {
    StatusCode::OK
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c().await.expect("setup ctrl-c signal");
}
