extern crate vachelin;

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use mongodb::bson::doc;
use serde::Serialize;
use serde_json::Value;
use tower::ServiceExt;
use vachelin::app;

#[derive(Serialize)]
struct MockData {
    name: String,
    distance: i32,
    recommanded: i32,
    menus: Vec<String>,
}

#[tokio::test]
async fn fail_create_diner_if_has_no_body() {
    let app = app().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/diners")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let content_type = response
        .headers()
        .get("Content-Type")
        .unwrap()
        .to_str()
        .unwrap();
    assert_eq!(content_type.contains(&"application/json"), true);
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let body: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(&body["message"], "Data not found");
}
