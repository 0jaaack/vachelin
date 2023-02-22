extern crate vachelin;

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;
use vachelin::app;

#[tokio::test]
async fn hello_rust() {
    let app = app().await;

    let response = app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();
    let content_type = response
        .headers()
        .get("Content-Type")
        .unwrap()
        .to_str()
        .unwrap();
    assert_eq!(content_type.contains(&"text/plain"), true);
    assert_eq!(response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    assert_eq!(&body[..], b"Hello, Rust!");
}
