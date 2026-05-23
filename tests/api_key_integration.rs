mod support;

use axum::http::{Method, StatusCode};
use diesel::{QueryDsl, RunQueryDsl};
use serde_json::{Value, json};
use tower::util::ServiceExt;
use uuid::Uuid;

use rust_api::{
    app::build_app,
    domain::api_key_secret::{hash_api_key_value, is_hashed_api_key_value},
    infrastructure::persistence::{
        database::connect_and_migrate,
        entities::{api_key::NewApiKeyRow, user::NewUserRow},
        schema::{api_keys, users},
    },
};

use crate::support::{
    cleanup_database, create_user, empty_request, json_request, parse_uuid_field, response_json,
    test_database_url,
};

#[tokio::test]
async fn create_api_key_returns_raw_value_once_and_stores_hash() {
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
                "label": "local-dev",
            }),
        ))
        .await
    {
        Ok(response) => response,
        Err(err) => unreachable!("request failed: {err}"),
    };

    assert_eq!(response.status(), StatusCode::CREATED);
    let body = response_json(response).await;
    let key_id = parse_uuid_field(&body, "id");
    let key_value = match body.get("key_value").and_then(Value::as_str) {
        Some(value) => value.to_string(),
        None => unreachable!("create response is missing key_value"),
    };

    assert!(key_value.starts_with("rk_"));

    let pool = match connect_and_migrate(&database_url).await {
        Ok(pool) => pool,
        Err(err) => unreachable!("failed to reconnect database: {err}"),
    };
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(err) => unreachable!("failed to get database connection: {err}"),
    };
    let stored_hash = match api_keys::table
        .find(key_id.to_string())
        .select(api_keys::key_hash)
        .first::<String>(&mut conn)
    {
        Ok(hash) => hash,
        Err(err) => unreachable!("failed to load stored hash: {err}"),
    };

    assert_eq!(stored_hash, hash_api_key_value(&key_value));
    assert!(is_hashed_api_key_value(&stored_hash));
    assert_ne!(stored_hash, key_value);

    drop(conn);
    drop(pool);
    drop(app);
    cleanup_database(&database_path);
}

#[tokio::test]
async fn list_api_keys_returns_redacted_metadata() {
    let (database_url, database_path) = test_database_url();
    let app = match build_app(&database_url).await {
        Ok(app) => app,
        Err(err) => unreachable!("failed to build app: {err}"),
    };

    let user_id = create_user(&app).await;
    let create_response = match app
        .clone()
        .oneshot(json_request(
            Method::POST,
            "/api-keys",
            json!({
                "user_id": user_id,
                "label": "ci",
            }),
        ))
        .await
    {
        Ok(response) => response,
        Err(err) => unreachable!("request failed: {err}"),
    };

    assert_eq!(create_response.status(), StatusCode::CREATED);

    let list_response = match app
        .clone()
        .oneshot(empty_request(Method::GET, "/api-keys"))
        .await
    {
        Ok(response) => response,
        Err(err) => unreachable!("request failed: {err}"),
    };

    assert_eq!(list_response.status(), StatusCode::OK);
    let body = response_json(list_response).await;
    let items = match body.as_array() {
        Some(items) => items,
        None => unreachable!("list response is not an array"),
    };

    assert_eq!(items.len(), 1);
    assert!(items[0].get("key_value").is_none());
    assert_eq!(items[0].get("label").and_then(Value::as_str), Some("ci"));
    assert_eq!(
        items[0].get("revoked").and_then(Value::as_bool),
        Some(false)
    );

    drop(app);
    cleanup_database(&database_path);
}

#[tokio::test]
async fn revoke_api_key_marks_key_revoked_without_exposing_secret() {
    let (database_url, database_path) = test_database_url();
    let app = match build_app(&database_url).await {
        Ok(app) => app,
        Err(err) => unreachable!("failed to build app: {err}"),
    };

    let user_id = create_user(&app).await;
    let create_response = match app
        .clone()
        .oneshot(json_request(
            Method::POST,
            "/api-keys",
            json!({
                "user_id": user_id,
                "label": "deploy",
            }),
        ))
        .await
    {
        Ok(response) => response,
        Err(err) => unreachable!("request failed: {err}"),
    };

    let created_body = response_json(create_response).await;
    let key_id = parse_uuid_field(&created_body, "id");

    let revoke_response = match app
        .clone()
        .oneshot(empty_request(
            Method::POST,
            &format!("/api-keys/{key_id}/revoke"),
        ))
        .await
    {
        Ok(response) => response,
        Err(err) => unreachable!("request failed: {err}"),
    };

    assert_eq!(revoke_response.status(), StatusCode::OK);
    let revoked_body = response_json(revoke_response).await;
    assert!(revoked_body.get("key_value").is_none());
    assert_eq!(
        revoked_body.get("revoked").and_then(Value::as_bool),
        Some(true)
    );

    let list_response = match app
        .clone()
        .oneshot(empty_request(Method::GET, "/api-keys"))
        .await
    {
        Ok(response) => response,
        Err(err) => unreachable!("request failed: {err}"),
    };
    let list_body = response_json(list_response).await;
    let items = match list_body.as_array() {
        Some(items) => items,
        None => unreachable!("list response is not an array"),
    };

    assert_eq!(items[0].get("revoked").and_then(Value::as_bool), Some(true));
    assert!(items[0].get("key_value").is_none());

    drop(app);
    cleanup_database(&database_path);
}

#[tokio::test]
async fn reconnect_backfills_legacy_plaintext_api_keys() {
    let (database_url, database_path) = test_database_url();
    let legacy_key_value = "rk_legacy_plaintext_secret";
    let user_id = Uuid::now_v7();
    let api_key_id = Uuid::now_v7();

    let pool = match connect_and_migrate(&database_url).await {
        Ok(pool) => pool,
        Err(err) => unreachable!("failed to initialize database: {err}"),
    };
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(err) => unreachable!("failed to get database connection: {err}"),
    };

    match diesel::insert_into(users::table)
        .values(&NewUserRow {
            id: &user_id.to_string(),
            name: "Legacy User",
            email: "legacy@example.com",
        })
        .execute(&mut conn)
    {
        Ok(_) => {}
        Err(err) => unreachable!("failed to insert user: {err}"),
    }

    match diesel::insert_into(api_keys::table)
        .values(&NewApiKeyRow {
            id: &api_key_id.to_string(),
            user_id: &user_id.to_string(),
            key_hash: legacy_key_value,
            label: Some("legacy"),
            revoked: false,
        })
        .execute(&mut conn)
    {
        Ok(_) => {}
        Err(err) => unreachable!("failed to insert legacy api key: {err}"),
    }

    drop(conn);
    drop(pool);

    let reconnected_pool = match connect_and_migrate(&database_url).await {
        Ok(pool) => pool,
        Err(err) => unreachable!("failed to reconnect database: {err}"),
    };
    let mut reconnected_conn = match reconnected_pool.get() {
        Ok(conn) => conn,
        Err(err) => unreachable!("failed to get database connection: {err}"),
    };
    let stored_hash = match api_keys::table
        .find(api_key_id.to_string())
        .select(api_keys::key_hash)
        .first::<String>(&mut reconnected_conn)
    {
        Ok(hash) => hash,
        Err(err) => unreachable!("failed to load stored hash: {err}"),
    };

    assert_eq!(stored_hash, hash_api_key_value(legacy_key_value));
    assert!(is_hashed_api_key_value(&stored_hash));

    drop(reconnected_conn);
    drop(reconnected_pool);
    cleanup_database(&database_path);
}
