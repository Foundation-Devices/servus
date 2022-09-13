// re-exports
pub use axum;
pub use clap;
pub use prometheus;
pub use serde;
pub use serde_json;
pub use sqlx;
pub use tokio;
pub use tower;
pub use tower_http;
pub use tracing;
// internal module exports
pub mod http;

use std::net::SocketAddr;
use tracing_subscriber::EnvFilter;

#[derive(clap::Args, Debug)]
pub struct Config {
    #[clap(
        long = "servus-http-address",
        env = "SERVUS_HTTP_ADDRESS",
        default_value = "0.0.0.0:8000"
    )]
    pub http_address: SocketAddr,

    #[clap(
        long = "servus-metrics-address",
        env = "SERVUS_METRICS_ADDRESS",
        default_value = "0.0.0.0:9000"
    )]
    pub metrics_address: SocketAddr,

    #[clap(
        long = "servus-log-json",
        env = "SERVUS_LOG_JSON",
        value_parser,
        default_value_t = false
    )]
    pub log_json: bool,

    #[clap(long = "servus-database-url", env = "SERVUS_DATABASE_URL")]
    pub database_url: Option<String>,
}

pub fn init(config: &Config) {
    if config.log_json {
        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .json()
            .init();
    } else {
        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .compact()
            .init();
    }
}
