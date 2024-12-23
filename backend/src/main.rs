use anyhow::{Context, Result};
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::post;
use axum::{Json, Router};

use self::database::Database;

mod database;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let database = Database::new("/opt/data").context("failed to init database")?;

    let router = Router::new()
        .route("/submit", post(submit))
        .with_state(database);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:80")
        .await
        .context("failed to bind addr")?;

    axum::serve(listener, router)
        .await
        .context("failed to serve")?;

    Ok(())
}

#[derive(serde::Deserialize)]
pub struct SubmitBody {
    name: Option<String>,
    phone: u64,
}

async fn submit(
    State(database): State<Database>,
    Json(body): Json<SubmitBody>,
) -> Result<StatusCode, StatusCode> {
    let name = body.name.as_deref();
    let phone = body.phone;

    if !(70000000000..=79999999999).contains(&phone) {
        return Err(StatusCode::BAD_REQUEST);
    }

    database.add_submit(name, phone).map_err(|e| {
        eprintln!("failed to add submit: {e:?}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(StatusCode::OK)
}
