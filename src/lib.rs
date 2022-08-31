// re-exports
pub use axum;
pub use clap;
pub use prometheus;
pub use tokio;

// internal module exports
pub mod http;

#[derive(clap::Args, Debug)]
pub struct Config {
    #[clap(
        long = "rustkit-http-address",
        env = "RUSTKIT_HTTP_ADDRESS",
        default_value = "0.0.0.0:8000"
    )]
    pub http_address: std::net::SocketAddr,

    #[clap(
        long = "rustkit-metrics-address",
        env = "RUSTKIT_METRICS_ADDRESS",
        default_value = "0.0.0.0:9000"
    )]
    pub metrics_address: std::net::SocketAddr,
}
