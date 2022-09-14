use servus::{
    axum::{extract::Path, http::StatusCode, response::IntoResponse, routing::get, Router},
    clap::Parser,
    tokio,
};

#[derive(Parser)]
struct AppConfig {
    #[clap(flatten)]
    servus: servus::Config,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = AppConfig::parse();
    servus::init(&config.servus);

    let router = Router::new().route("/echo/:message", get(echo));

    // start the axum server
    // Note, we pass the `metrics_address` parameter value as `None` to imply we don't want to
    // start the metrics server. Also, the `state` parameter is the unit type `()`, meaning we have
    // no global state and all handlers are stateless.
    servus::http::serve(config.servus.http_address, None, router, ()).await;

    Ok(())
}

async fn echo(Path(message): Path<String>) -> impl IntoResponse {
    (StatusCode::OK, message)
}
