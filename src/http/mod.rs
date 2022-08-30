pub mod metrics;

use axum::{
    extract::Extension,
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::Response,
    routing, Router, Server,
};
use prometheus::{Encoder, TextEncoder};
use std::net::SocketAddr;
use tokio::time::Instant;

// Entrypoint for serving HTTP requests.
// Creates two instances of a `hyper::Server`, one for application routes, and another for metrics
// output. The separate server for metrics prevents incoming connections from accessing
// that route, keeping it internal to the deployed environment.
pub async fn serve<S>(addrs: (SocketAddr, SocketAddr), state: S, router: Router)
where
    S: Send + Sync + Clone + 'static,
{
    // create primary application router and server,
    // applying handler state and default middleware
    let r = router
        .layer(Extension(state))
        .layer(middleware::from_fn(metrics_middleware));

    let app = Server::bind(&addrs.0)
        .serve(r.into_make_service())
        .with_graceful_shutdown(shutdown_signal());

    // create metrics router and server
    let r = Router::new().route("/metrics", routing::get(metrics_handler));

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

async fn metrics_middleware<B>(req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    let method = String::from(req.method().as_str());
    let path = String::from(req.uri().path());

    let start = Instant::now();
    let resp = next.run(req).await;
    let end = Instant::now();

    // record HTTP metric(s)
    metrics::HTTP_REQUEST_DURATION
        .with_label_values(&[&method, &path, resp.status().as_str()])
        .observe(end.duration_since(start).as_secs_f64());

    Ok(resp)
}

// write metrics from default prometheus register
async fn metrics_handler() -> String {
    let mut buffer = Vec::new();
    let encoder = TextEncoder::new();

    let metrics = prometheus::gather();
    encoder.encode(&metrics, &mut buffer).unwrap();

    String::from_utf8(buffer.clone()).unwrap()
}
