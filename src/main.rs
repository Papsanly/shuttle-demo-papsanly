use axum::{routing::get, Router};
use shuttle_axum::ShuttleAxum;
use shuttle_secrets::{SecretStore, Secrets};

async fn hello_world() -> &'static str {
    "Bye, world!"
}

#[shuttle_runtime::main]
async fn main(#[Secrets] _secret_store: SecretStore) -> ShuttleAxum {
    let router = Router::new().route("/", get(hello_world));
    Ok(router.into())
}
