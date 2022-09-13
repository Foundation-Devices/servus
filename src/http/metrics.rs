use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use once_cell::sync::Lazy;
use prometheus::{register_histogram_vec, Encoder, HistogramVec, TextEncoder};
use tokio::time::Instant;

pub static HTTP_REQUEST_DURATION: Lazy<HistogramVec> = Lazy::new(|| {
    register_histogram_vec!(
        "servus_http_request_duration",
        "HTTP request duration in seconds as a histogram, by method, path, and status",
        &["method", "path", "status"]
    )
    .unwrap()
});

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

pub async fn handler() -> String {
    let mut buffer = Vec::new();
    let encoder = TextEncoder::new();

    let metrics = prometheus::gather();
    encoder.encode(&metrics, &mut buffer).unwrap();

    String::from_utf8(buffer.clone()).unwrap()
}
