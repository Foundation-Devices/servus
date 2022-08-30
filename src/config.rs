use clap::Args;
use std::net::SocketAddr;

#[derive(Args, Debug)]
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
}
