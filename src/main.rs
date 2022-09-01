use rustkit::axum::{extract::Extension, extract::Path, http::StatusCode, routing::get, Router};
use rustkit::clap::Parser;
use rustkit::tokio;
use rustkit::tracing::{info, instrument, warn};
use std::time::Duration;
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
    rustkit::init(&config.rustkit);

    let state = Arc::new(config.response.clone());
    let router = Router::new().route("/:status", get(handler));
    rustkit::http::serve(
        (config.rustkit.http_address, config.rustkit.metrics_address),
        state,
        router,
    )
    .await;
}

#[instrument(skip(state))]
async fn handler(
    Extension(state): Extension<Arc<String>>,
    Path(status): Path<String>,
) -> (StatusCode, String) {
    info!(msg = "got handler request!", label = "test");
    warn!(msg = "about to do some work");

    // do some "work"
    do_work(Duration::from_secs(1)).await;

    let status = StatusCode::from_str(&status).unwrap();
    (status, state.to_string())
}

#[instrument]
async fn do_work(duration: Duration) {
    tokio::time::sleep(duration).await;
}
