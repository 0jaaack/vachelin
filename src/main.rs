use axum::{routing::get, Router};
use dotenv::dotenv;
use mongodb::{options::ClientOptions, Client};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let mongo_uri: String = std::env::var("MONGO_URI")
        .expect("Failed to load `MONGO_MAX_POOL_SIZE` environment variable.");

    let mut client_options = ClientOptions::parse(mongo_uri).await.unwrap();
    client_options.app_name = Some("mytimes".to_string());
    let client = Client::with_options(client_options).unwrap();

    let app = Router::new().route("/", get(|| async { "Hello, Rust!" }));
    let app = app.with_state(client);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
