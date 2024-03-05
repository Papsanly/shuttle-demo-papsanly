use axum::{routing::get, Router};
use shuttle_axum::ShuttleAxum;
use shuttle_runtime::CustomError;
use shuttle_shared_db::Postgres;
use sqlx::{Executor, PgPool};

async fn hello_world() -> &'static str {
    "Bye, world!"
}

#[shuttle_runtime::main]
async fn main(#[Postgres] pool: PgPool) -> ShuttleAxum {
    pool.execute(include_str!("schema.sql"))
        .await
        .map_err(CustomError::new)?;

    let router = Router::new().route("/", get(hello_world));
    Ok(router.into())
}
