use once_cell::sync::Lazy;
use prometheus::{register_histogram_vec, HistogramVec};

pub static HTTP_REQUEST_DURATION: Lazy<HistogramVec> = Lazy::new(|| {
    register_histogram_vec!(
        "http_request_duration",
        "HTTP request duration in seconds as a histogram, by method, path, and status",
        &["method", "path", "status"]
    )
    .unwrap()
});
