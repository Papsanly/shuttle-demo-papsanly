use axum::{routing::get, Router};
use shuttle_secrets::SecretStore;

async fn hello_world() -> &'static str {
    "Bye, world!"
}

#[shuttle_runtime::main]
async fn main(#[shuttle_secrets::Secrets] _secret_store: SecretStore) -> shuttle_axum::ShuttleAxum {
    let router = Router::new().route("/", get(hello_world));
    Ok(router.into())
}
