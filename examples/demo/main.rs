use servus::axum::{
    extract::{self, Extension},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use servus::clap::Parser;
use servus::serde;
use servus::serde_json::json;
use servus::sqlx;
use servus::tokio;
use servus::tracing::{error, info};
use std::sync::Arc;

#[derive(Parser, Debug)]
struct AppConfig {
    #[clap(flatten)]
    servus: servus::Config,

    #[clap(short, long, env = "TEST_RESPONSE", default_value = "ok!")]
    response: String,
}

struct AppState {
    pool: sqlx::postgres::PgPool,
}

impl AppState {
    fn new(pool: sqlx::postgres::PgPool) -> Self {
        Self { pool }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // parse CLI config
    let config = AppConfig::parse();
    // setup logging
    servus::init(&config.servus);

    let state = if let Some(url) = &config.servus.database_url {
        // if we have a database URL, create a connection pool
        // we assume migrations have already been applied
        Arc::new(AppState::new(
            sqlx::postgres::PgPoolOptions::new().connect(url).await?,
        ))
    } else {
        // this typically would be handled in some kind of validation step on the config,
        // which servus cannot define, as it would be application-dependent
        return Err(anyhow::anyhow!("database url is needed for this demo!"));
    };

    let router = Router::new()
        .route("/message", post(post_message))
        .route("/message/all", get(get_messages));

    servus::http::serve(
        (config.servus.http_address, config.servus.metrics_address),
        state,
        router,
    )
    .await;

    Ok(())
}

#[derive(serde::Deserialize, serde::Serialize)]
struct Message {
    author: String,
    message: String,
}

async fn post_message(
    Extension(state): Extension<Arc<AppState>>,
    extract::Json(payload): extract::Json<Message>,
) -> StatusCode {
    info!(
        message = "got post message request!",
        author = payload.author,
        msg = payload.message
    );

    if let Err(e) = sqlx::query!(
        "INSERT INTO guestbook (author, message) VALUES ($1, $2)",
        payload.author,
        payload.message
    )
    .execute(&state.pool)
    .await
    {
        error!(msg = "error inserting into table", err = e.to_string());
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    StatusCode::OK
}

async fn get_messages(Extension(state): Extension<Arc<AppState>>) -> impl IntoResponse {
    info!(message = "got get messages request!");

    let q = sqlx::query!("select * from guestbook")
        .fetch_all(&state.pool)
        .await;

    match q {
        Err(e) => {
            error!(msg = "error getting messages", err = e.to_string());
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "db error" })),
            )
        }
        Ok(r) => (
            StatusCode::OK,
            Json(
                json!({ "messages": r.into_iter().map(|m| Message{ author: m.author, message: m.message }).collect::<Vec<Message>>() }),
            ),
        ),
    }
}
