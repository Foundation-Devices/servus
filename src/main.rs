use axum::{extract::Path, http::StatusCode, routing::get, Router};
use std::{net::SocketAddr, str::FromStr};

#[tokio::main]
async fn main() {
    let addrs: (SocketAddr, SocketAddr) =
        (([127, 0, 0, 1], 8000).into(), ([127, 0, 0, 1], 9000).into());

    let router = Router::new().route("/:status", get(handler));
    rustkit::http::serve(addrs, router).await;
}

async fn handler(Path(status): Path<String>) -> (StatusCode, &'static str) {
    let status = StatusCode::from_str(&status).unwrap();
    (status, "ok!")
}
