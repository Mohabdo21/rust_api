#![allow(dead_code)]

use std::{env, fs, path::PathBuf};

use axum::{
    body::{Body, to_bytes},
    http::{Method, Request, StatusCode},
};
use serde_json::{Value, json};
use tower::util::ServiceExt;
use uuid::Uuid;

pub fn test_database_url() -> (String, PathBuf) {
    let path = env::temp_dir().join(format!("rust_api_test_{}.db", Uuid::new_v4()));
    (format!("sqlite://{}", path.display()), path)
}

pub fn json_request(method: Method, uri: &str, body: Value) -> Request<Body> {
    match Request::builder()
        .method(method)
        .uri(uri)
        .header("content-type", "application/json")
        .body(Body::from(body.to_string()))
    {
        Ok(request) => request,
        Err(err) => unreachable!("failed to build request: {err}"),
    }
}

pub fn raw_request(
    method: Method,
    uri: &str,
    body: &str,
    content_type: Option<&str>,
) -> Request<Body> {
    let mut builder = Request::builder().method(method).uri(uri);
    if let Some(content_type) = content_type {
        builder = builder.header("content-type", content_type);
    }

    match builder.body(Body::from(body.to_string())) {
        Ok(request) => request,
        Err(err) => unreachable!("failed to build request: {err}"),
    }
}

pub fn empty_request(method: Method, uri: &str) -> Request<Body> {
    match Request::builder()
        .method(method)
        .uri(uri)
        .body(Body::empty())
    {
        Ok(request) => request,
        Err(err) => unreachable!("failed to build request: {err}"),
    }
}

pub async fn response_json(response: axum::response::Response) -> Value {
    let bytes = match to_bytes(response.into_body(), usize::MAX).await {
        Ok(bytes) => bytes,
        Err(err) => unreachable!("failed to read response body: {err}"),
    };

    match serde_json::from_slice::<Value>(&bytes) {
        Ok(value) => value,
        Err(err) => unreachable!("failed to decode response body: {err}"),
    }
}

pub async fn create_user(app: &axum::Router) -> Uuid {
    let response = match app
        .clone()
        .oneshot(json_request(
            Method::POST,
            "/users",
            json!({
                "name": "Alice",
                "email": format!("alice-{}@example.com", Uuid::new_v4()),
            }),
        ))
        .await
    {
        Ok(response) => response,
        Err(err) => unreachable!("request failed: {err}"),
    };

    assert_eq!(response.status(), StatusCode::CREATED);
    let body = response_json(response).await;

    parse_uuid_field(&body, "id")
}

pub fn parse_uuid_field(body: &Value, field: &str) -> Uuid {
    let value = match body.get(field).and_then(Value::as_str) {
        Some(value) => value,
        None => unreachable!("missing uuid field: {field}"),
    };

    match Uuid::parse_str(value) {
        Ok(uuid) => uuid,
        Err(err) => unreachable!("invalid uuid in field {field}: {err}"),
    }
}

pub fn cleanup_database(path: &PathBuf) {
    let _ = fs::remove_file(path);
}
