use axum::{
    Router,
    routing::{delete, get, post},
};

use crate::api::handlers::{
    AppState, create_api_key, create_user, delete_api_key, get_user, health, list_api_keys,
    list_user_api_keys, list_users, revoke_api_key,
};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/users", post(create_user).get(list_users))
        .route("/users/{id}", get(get_user))
        .route("/users/{id}/api-keys", get(list_user_api_keys))
        .route("/api-keys", post(create_api_key).get(list_api_keys))
        .route("/api-keys/{id}", delete(delete_api_key))
        .route("/api-keys/{id}/revoke", post(revoke_api_key))
        .with_state(state)
}
