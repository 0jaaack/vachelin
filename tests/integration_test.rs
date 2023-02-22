extern crate vachelin;

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;
use vachelin::app;

#[tokio::test]
async fn hello_world() {
    let app = app().await;

    let response = app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(
        response
            .headers()
            .get("Content-Type")
            .unwrap()
            .to_str()
            .unwrap()
            .contains(&"text/plain"),
        true
    );
    assert_eq!(response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    assert_eq!(&body[..], b"Hello, Rust!");
}
