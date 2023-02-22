use axum::{routing::get, Router};
use dotenv::dotenv;
use mongodb::{options::ClientOptions, Client};
use std::env;

pub async fn app() -> Router {
    let client = client().await;

    Router::new()
        .route("/", get(|| async { "Hello, Rust!" }))
        .with_state(client)
}

pub async fn client() -> Client {
    dotenv().ok();

    let mongo_uri: String =
        env::var("MONGO_URI").expect("Failed to load `MONGO_URI` environment variable.");

    let mut client_options = ClientOptions::parse(mongo_uri).await.unwrap();
    client_options.app_name = Some("vachelin".to_string());

    Client::with_options(client_options).unwrap()
}
