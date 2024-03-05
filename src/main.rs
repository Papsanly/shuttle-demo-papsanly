use axum::extract::{Path, State};
use axum::{
    response::{Redirect, Result},
    routing::{get, post},
    Router,
};
use shuttle_axum::ShuttleAxum;
use shuttle_runtime::CustomError;
use shuttle_shared_db::Postgres;
use sqlx::{Executor, PgPool};
use std::sync::Arc;

struct AppState {
    pool: PgPool,
}

async fn redirect(Path(id): Path<u32>, State(state): State<Arc<AppState>>) -> Redirect {
    unimplemented!()
}

async fn shorten(State(state): State<Arc<AppState>>, url: String) -> String {
    unimplemented!()
}

#[shuttle_runtime::main]
async fn main(#[Postgres] pool: PgPool) -> ShuttleAxum {
    pool.execute(include_str!("schema.sql"))
        .await
        .map_err(CustomError::new)?;

    let state = Arc::new(AppState { pool });

    let router = Router::new()
        .route("/", post(shorten))
        .route("/", get(redirect))
        .with_state(state);

    Ok(router.into())
}
