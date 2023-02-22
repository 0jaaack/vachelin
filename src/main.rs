extern crate vachelin;

use axum::Router;

#[tokio::main]
async fn main() {
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app().await.into_make_service())
        .await
        .unwrap();
}

pub async fn app() -> Router {
    vachelin::app().await
}
