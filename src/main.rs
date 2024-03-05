use axum::{
    extract::{Host, Path, State},
    http::StatusCode,
    response::{Redirect, Result},
    routing::{get, post},
    Router,
};
use nanoid::nanoid;
use shuttle_axum::ShuttleAxum;
use shuttle_runtime::CustomError;
use shuttle_shared_db::Postgres;
use sqlx::{Error, Executor, PgPool};
use std::sync::Arc;
use url::Url;

struct AppState {
    pool: PgPool,
}

async fn redirect(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<Redirect, StatusCode> {
    let (url,): (String,) = sqlx::query_as("select url from urls where id = $1")
        .bind(id)
        .fetch_one(&state.pool)
        .await
        .map_err(|e| match e {
            Error::RowNotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        })?;
    Ok(Redirect::to(&url))
}

async fn shorten(
    State(state): State<Arc<AppState>>,
    Host(host): Host,
    url: String,
) -> Result<String, StatusCode> {
    let id = &nanoid!(6);
    let p_url = Url::parse(&url).map_err(|_| StatusCode::UNPROCESSABLE_ENTITY)?;
    sqlx::query("insert into urls(id, url) values ($1, $2)")
        .bind(id)
        .bind(p_url.as_str())
        .execute(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(format!("https://{host}/{id}"))
}

#[shuttle_runtime::main]
async fn main(#[Postgres] pool: PgPool) -> ShuttleAxum {
    pool.execute(include_str!("schema.sql"))
        .await
        .map_err(CustomError::new)?;

    let state = Arc::new(AppState { pool });

    let router = Router::new()
        .route("/shorten", post(shorten))
        .route("/:id", get(redirect))
        .with_state(state);

    Ok(router.into())
}
