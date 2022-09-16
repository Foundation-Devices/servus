// SPDX-FileCopyrightText: 2022 Foundation Devices Inc. <hello@foundationdevices.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Standard metrics applied to all application HTTP routes.

use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use once_cell::sync::Lazy;
use prometheus::{register_histogram_vec, Encoder, HistogramVec, TextEncoder};
use tokio::time::Instant;

/// Prometheus histogram metrics measuring the request duration.
///
/// Labeled by request `method`, `path`, and response `status` code.
pub static HTTP_REQUEST_DURATION: Lazy<HistogramVec> = Lazy::new(|| {
    register_histogram_vec!(
        "servus_http_request_duration",
        "HTTP request duration in seconds as a histogram, by method, path, and status",
        &["method", "path", "status"]
    )
    .unwrap()
});

/// `axum` middleware function applied to all application HTTP routes.
///
/// Records the `HTTP_REQUEST_DURATION` histogram metric after each route handler has completed.
pub async fn middleware<B>(req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    let method = String::from(req.method().as_str());
    let path = String::from(req.uri().path());

    let start = Instant::now();
    let resp = next.run(req).await;
    let end = Instant::now();

    HTTP_REQUEST_DURATION
        .with_label_values(&[&method, &path, resp.status().as_str()])
        .observe(end.duration_since(start).as_secs_f64());

    Ok(resp)
}

/// Request handler used for the `GET /metrics` route on the (optional) metrics server.
///
/// Gathers all records Prometheus metrics emitted by the application and responds to the scraping
/// process accessing the metrics route.
pub async fn handler() -> String {
    let mut buffer = Vec::new();
    let encoder = TextEncoder::new();

    let metrics = prometheus::gather();
    encoder.encode(&metrics, &mut buffer).unwrap();

    String::from_utf8(buffer.clone()).unwrap()
}
