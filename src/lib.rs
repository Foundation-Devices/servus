// SPDX-FileCopyrightText: 2022 Foundation Devices Inc. <hello@foundationdevices.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

//! `servus` is a library of common structures and functions for building backend web services in Rust.
//! By bringing together a set of great libraries and exposing a thin layer, it hopes to eliminate common boilerplate
//! code needed in production-ready backend services, such as HTTP routing, SQL connectivity, metrics, and logging.
//!
//! Many of the dependencies of `servus` are re-exported for convenience.
//!
//! ```rust
//! use servus::{
//!     axum::{extract::Path, http::StatusCode, response::IntoResponse, routing::get, Router},
//!     clap::Parser,
//!     tokio,
//! };
//!
//! #[derive(Parser)]
//! struct AppConfig {
//!     #[clap(flatten)]
//!     servus: servus::Config,
//! }
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let config = AppConfig::parse();
//!     servus::init(&config.servus);
//!
//!     let router = Router::new().route("/echo/:message", get(echo));
//!
//!     // start the axum server
//!     // Note, we pass the `metrics_address` parameter value as `None` to imply we don't want to
//!     // start the metrics server. Also, the `state` parameter is the unit type `()`, meaning we have
//!     // no global state and all handlers are stateless.
//!     servus::http::serve(config.servus.http_address, None, router, ()).await;
//!
//!     Ok(())
//! }
//!
//! async fn echo(Path(message): Path<String>) -> impl IntoResponse {
//!     (StatusCode::OK, message)
//! }
//! ```

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

/// Primary `servus` configuration struct.
///
/// It derives `clap::Args`, so it is intended
/// to be used in your application's config struct as a "flattend" set of command-line and
/// environmental arguments.
///
/// It can be used in your application code like so,
/// ```
/// use servus::{clap, Config};
///
/// #[derive(clap::Parser)]
/// struct AppConfig {
///     #[clap(flatten)]
///     servus: Config,
///
///     // you can add more application-specific configuration as well...
///     #[clap(short, long)]
///     my_application_value: String,
/// }
/// ```
#[derive(clap::Args, Debug)]
pub struct Config {
    /// `SocketAddr` the application HTTP router should listen on.
    ///
    /// Populated from a CLI flag `--servus-http-address` or env var `SERVUS_HTTP_ADDRESS`.
    ///
    /// Defaults to `0.0.0.0:8000`.
    #[clap(
        long = "servus-http-address",
        env = "SERVUS_HTTP_ADDRESS",
        default_value = "0.0.0.0:8000"
    )]
    pub http_address: SocketAddr,

    /// `SocketAddr` the metrics HTTP router should listen on.
    /// This generates a separate instance of the `axum::Server` only used for metrics and
    /// a simple healthcheck.
    ///
    /// Populated from a CLI flag `--servus-metrics-address` or env var `SERVUS_METRICS_ADDRESS`.
    ///
    /// Defaults to `0.0.0.0:9000`.
    #[clap(
        long = "servus-metrics-address",
        env = "SERVUS_METRICS_ADDRESS",
        default_value = "0.0.0.0:9000"
    )]
    pub metrics_address: SocketAddr,

    /// If `true`, all log output will be in JSON format. Defaults to `false`.
    ///
    /// Populated from a CLI flag `--servus-log-json` or env var `SERVUS_LOG_JSON`.
    #[clap(
        long = "servus-log-json",
        env = "SERVUS_LOG_JSON",
        value_parser,
        default_value_t = false
    )]
    pub log_json: bool,

    /// Provided as a convenience for applications that require a SQL database connection.
    ///
    /// `servus` will not do anything with it automatically, since not all applications need a
    /// database connection. But, your application can use it at runtime via `config.database_url`.
    ///
    /// Populated from a CLI flag `--servus-database-url` or env var `SERVUS_DATABASE_URL`.
    #[clap(long = "servus-database-url", env = "SERVUS_DATABASE_URL")]
    pub database_url: Option<String>,
}

/// Initializes application-wide configuration based on a parsed `Config` struct.
///
/// Right now, this simply looks at the `log_json` field of the `Config` struct and initializes a
/// tracing subscriber for log output based on that value.
///
/// It should be called early in your application's startup routine.
///
/// ```rust
/// use servus::{clap, Config, tokio};
///
/// #[derive(clap::Parser)]
/// struct AppConfig {
///     #[clap(flatten)]
///     servus: Config,
///
///     // ... application-specifc config ...
/// }
///
///
/// #[tokio::main]
/// async fn main() {
///     let config = AppConfig::parse();
///     servus::init(&config.servus);
///
///     // ... continue application startup ...
/// }
/// ```
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
