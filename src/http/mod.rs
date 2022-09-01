pub mod metrics;

use axum::{extract::Extension, middleware, routing, Router, Server};
use std::net::SocketAddr;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing::Level;

// Entrypoint for serving HTTP requests.
// Creates two instances of a `hyper::Server`, one for application routes, and another for metrics
// output. The separate server for metrics prevents incoming connections from accessing
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

    // create metrics router and server
    let r = Router::new().route("/metrics", routing::get(metrics::handler));

    let metrics = Server::bind(&addrs.1)
        .serve(r.into_make_service())
        .with_graceful_shutdown(shutdown_signal());

    // wait on both server instances,
    // returning early if one results in an error
    if let Err(e) = tokio::try_join!(app, metrics) {
        println!("server error = {}", e);
    }
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c().await.expect("setup ctrl-c signal");
}
