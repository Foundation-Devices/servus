use axum::{extract::Path, http::StatusCode, routing::get, Router};
use clap::Parser;
use std::str::FromStr;

#[derive(Parser, Debug)]
struct AppConfig {
    #[clap(flatten)]
    rustkit: rustkit::config::Config,

    #[clap(short, long, env = "TEST_RESPONSE", default_value = "ok!")]
    response: String,
}

#[tokio::main]
async fn main() {
    let config = AppConfig::parse();
    println!("config = {:?}", config);

    let router = Router::new().route("/:status", get(handler));
    rustkit::http::serve(
        (config.rustkit.http_address, config.rustkit.metrics_address),
        router,
    )
    .await;
}

async fn handler(Path(status): Path<String>) -> (StatusCode, &'static str) {
    let status = StatusCode::from_str(&status).unwrap();
    (status, "ok!")
}
