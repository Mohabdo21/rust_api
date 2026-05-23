mod support;

use axum::http::{Method, StatusCode};
use serde_json::json;
use tower::util::ServiceExt;

use rust_api::app::build_app;

use crate::support::{
    cleanup_database, create_user, json_request, raw_request, response_json, test_database_url,
};

#[tokio::test]
async fn duplicate_email_returns_conflict_response() {
    let (database_url, database_path) = test_database_url();
    let app = match build_app(&database_url).await {
        Ok(app) => app,
        Err(err) => unreachable!("failed to build app: {err}"),
    };

    let first_response = match app
        .clone()
        .oneshot(json_request(
            Method::POST,
            "/users",
            json!({
                "name": "Alice",
                "email": "alice@example.com",
            }),
        ))
        .await
    {
        Ok(response) => response,
        Err(err) => unreachable!("request failed: {err}"),
    };
    assert_eq!(first_response.status(), StatusCode::CREATED);

    let second_response = match app
        .clone()
        .oneshot(json_request(
            Method::POST,
            "/users",
            json!({
                "name": "Alice Again",
                "email": "alice@example.com",
            }),
        ))
        .await
    {
        Ok(response) => response,
        Err(err) => unreachable!("request failed: {err}"),
    };

    assert_eq!(second_response.status(), StatusCode::CONFLICT);
    let body = response_json(second_response).await;
    assert_eq!(
        body.get("error").and_then(|value| value.as_str()),
        Some("conflict")
    );
    assert_eq!(
        body.get("message").and_then(|value| value.as_str()),
        Some("email already exists")
    );

    drop(app);
    cleanup_database(&database_path);
}

#[tokio::test]
async fn invalid_email_returns_validation_error() {
    let (database_url, database_path) = test_database_url();
    let app = match build_app(&database_url).await {
        Ok(app) => app,
        Err(err) => unreachable!("failed to build app: {err}"),
    };

    let response = match app
        .clone()
        .oneshot(json_request(
            Method::POST,
            "/users",
            json!({
                "name": "Alice",
                "email": "not-an-email",
            }),
        ))
        .await
    {
        Ok(response) => response,
        Err(err) => unreachable!("request failed: {err}"),
    };

    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    let body = response_json(response).await;
    assert_eq!(
        body.get("error").and_then(|value| value.as_str()),
        Some("validation_error")
    );
    assert_eq!(
        body.get("message").and_then(|value| value.as_str()),
        Some("email must be a valid address")
    );

    drop(app);
    cleanup_database(&database_path);
}

#[tokio::test]
async fn blank_name_returns_validation_error() {
    let (database_url, database_path) = test_database_url();
    let app = match build_app(&database_url).await {
        Ok(app) => app,
        Err(err) => unreachable!("failed to build app: {err}"),
    };

    let response = match app
        .clone()
        .oneshot(json_request(
            Method::POST,
            "/users",
            json!({
                "name": "   ",
                "email": "alice@example.com",
            }),
        ))
        .await
    {
        Ok(response) => response,
        Err(err) => unreachable!("request failed: {err}"),
    };

    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    let body = response_json(response).await;
    assert_eq!(
        body.get("error").and_then(|value| value.as_str()),
        Some("validation_error")
    );
    assert_eq!(
        body.get("message").and_then(|value| value.as_str()),
        Some("name must not be empty")
    );

    drop(app);
    cleanup_database(&database_path);
}

#[tokio::test]
async fn blank_api_key_label_returns_validation_error() {
    let (database_url, database_path) = test_database_url();
    let app = match build_app(&database_url).await {
        Ok(app) => app,
        Err(err) => unreachable!("failed to build app: {err}"),
    };

    let user_id = create_user(&app).await;
    let response = match app
        .clone()
        .oneshot(json_request(
            Method::POST,
            "/api-keys",
            json!({
                "user_id": user_id,
                "label": "   ",
            }),
        ))
        .await
    {
        Ok(response) => response,
        Err(err) => unreachable!("request failed: {err}"),
    };

    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    let body = response_json(response).await;
    assert_eq!(
        body.get("error").and_then(|value| value.as_str()),
        Some("validation_error")
    );
    assert_eq!(
        body.get("message").and_then(|value| value.as_str()),
        Some("label must not be empty")
    );

    drop(app);
    cleanup_database(&database_path);
}

#[tokio::test]
async fn malformed_json_returns_bad_request_error() {
    let (database_url, database_path) = test_database_url();
    let app = match build_app(&database_url).await {
        Ok(app) => app,
        Err(err) => unreachable!("failed to build app: {err}"),
    };

    let response = match app
        .clone()
        .oneshot(raw_request(
            Method::POST,
            "/users",
            "{\"name\":\"Alice\",\"email\":",
            Some("application/json"),
        ))
        .await
    {
        Ok(response) => response,
        Err(err) => unreachable!("request failed: {err}"),
    };

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let body = response_json(response).await;
    assert_eq!(
        body.get("error").and_then(|value| value.as_str()),
        Some("invalid_request")
    );
    assert_eq!(
        body.get("message").and_then(|value| value.as_str()),
        Some("request body contains invalid JSON")
    );

    drop(app);
    cleanup_database(&database_path);
}

#[tokio::test]
async fn missing_required_field_returns_bad_request_error() {
    let (database_url, database_path) = test_database_url();
    let app = match build_app(&database_url).await {
        Ok(app) => app,
        Err(err) => unreachable!("failed to build app: {err}"),
    };

    let response = match app
        .clone()
        .oneshot(raw_request(
            Method::POST,
            "/users",
            "{\"name\":\"Alice\"}",
            Some("application/json"),
        ))
        .await
    {
        Ok(response) => response,
        Err(err) => unreachable!("request failed: {err}"),
    };

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let body = response_json(response).await;
    assert_eq!(
        body.get("error").and_then(|value| value.as_str()),
        Some("invalid_request")
    );
    assert_eq!(
        body.get("message").and_then(|value| value.as_str()),
        Some("request body is invalid")
    );

    drop(app);
    cleanup_database(&database_path);
}
