use axum::{routing::get, Router};
use shuttle_axum::ShuttleAxum;

async fn hello_world() -> &'static str {
    "Bye, world!"
}

#[shuttle_runtime::main]
async fn main() -> ShuttleAxum {
    let router = Router::new().route("/", get(hello_world));
    Ok(router.into())
}
