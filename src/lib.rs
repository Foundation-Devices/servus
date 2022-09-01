// re-exports
pub use axum;
pub use clap;
pub use prometheus;
pub use tokio;
pub use tracing;

use std::net::SocketAddr;

// internal module exports
pub mod http;

#[derive(clap::Args, Debug)]
pub struct Config {
    #[clap(
        long = "rustkit-http-address",
        env = "RUSTKIT_HTTP_ADDRESS",
        default_value = "0.0.0.0:8000"
    )]
    pub http_address: SocketAddr,

    #[clap(
        long = "rustkit-metrics-address",
        env = "RUSTKIT_METRICS_ADDRESS",
        default_value = "0.0.0.0:9000"
    )]
    pub metrics_address: SocketAddr,

    #[clap(
        long = "rustkit-log-json",
        env = "RUSTKIT_LOG_JSON",
        value_parser,
        default_value_t = false
    )]
    pub log_json: bool,
}

pub fn init(config: &Config) {
    println!("{:?}", config);
    if config.log_json {
        tracing_subscriber::fmt().json().init();
    } else {
        tracing_subscriber::fmt().compact().init();
    }
}
