use axum::{extract::Extension, extract::Path, http::StatusCode, routing::get, Router};
use clap::Parser;
use std::{str::FromStr, sync::Arc};

#[derive(Parser, Debug)]
struct AppConfig {
    #[clap(flatten)]
    rustkit: rustkit::Config,

    #[clap(short, long, env = "TEST_RESPONSE", default_value = "ok!")]
    response: String,
}

#[tokio::main]
async fn main() {
    let config = AppConfig::parse();
    println!("config = {:?}", config);

    let state = Arc::new(config.response.clone());
    let router = Router::new().route("/:status", get(handler));
    rustkit::http::serve(
        (config.rustkit.http_address, config.rustkit.metrics_address),
        state,
        router,
    )
    .await;
}

async fn handler(
    Extension(state): Extension<Arc<String>>,
    Path(status): Path<String>,
) -> (StatusCode, String) {
    let status = StatusCode::from_str(&status).unwrap();
    (status, state.to_string())
}
