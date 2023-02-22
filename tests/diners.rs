extern crate vachelin;

use axum::{
    body::Body,
    http::{self, Request, StatusCode},
};
use mongodb::{
    bson::{doc, Document},
    options::FindOneOptions,
};
use serde::Serialize;
use serde_json::{json, Value};
use tower::ServiceExt;
use vachelin::{app, client};

#[derive(Debug, Serialize)]
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

#[tokio::test]
async fn success_create_diner() {
    let app = app().await;

    let mock_data = MockData {
        name: "test diner".to_string(),
        distance: 10,
        recommanded: 0,
        menus: vec!["test menu".to_string()],
    };

    let response = app
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri("/diners")
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(serde_json::to_vec(&json!(&mock_data)).unwrap()))
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
    assert_eq!(response.status(), StatusCode::CREATED);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(&json["message"], "success");
    assert_eq!(&json["result"], &json!(&mock_data));

    let dinners_collection = client()
        .await
        .database("vachelin")
        .collection::<Document>("diners");
    let filter = doc! {
        "name": mock_data.name,
    };
    let options = FindOneOptions::default();
    let saved_dinner = dinners_collection.find_one(filter, options).await.unwrap();

    assert!(saved_dinner.is_some());
}
