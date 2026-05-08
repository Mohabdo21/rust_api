use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;

use crate::{
    api::{
        dto::{ApiKeyResponse, CreateApiKeyRequest, CreateUserRequest, UserResponse},
        error::ApiError,
    },
    application::{api_key_service::ApiKeyService, user_service::UserService},
};

#[derive(Clone)]
pub struct AppState {
    pub user_service: UserService,
    pub api_key_service: ApiKeyService,
}

pub async fn health() -> impl IntoResponse {
    (StatusCode::OK, "ok")
}

pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let user = state
        .user_service
        .create(payload.name, payload.email)
        .await?;
    Ok((StatusCode::CREATED, Json(UserResponse::from(user))))
}

pub async fn list_users(
    State(state): State<AppState>,
) -> Result<Json<Vec<UserResponse>>, ApiError> {
    let users = state.user_service.list().await?;
    Ok(Json(users.into_iter().map(UserResponse::from).collect()))
}

pub async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<UserResponse>, ApiError> {
    let user = state.user_service.get_by_id(id).await?;
    Ok(Json(UserResponse::from(user)))
}

pub async fn create_api_key(
    State(state): State<AppState>,
    Json(payload): Json<CreateApiKeyRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let key = state
        .api_key_service
        .create(payload.user_id, payload.label)
        .await?;

    Ok((StatusCode::CREATED, Json(ApiKeyResponse::from(key))))
}

pub async fn list_api_keys(
    State(state): State<AppState>,
) -> Result<Json<Vec<ApiKeyResponse>>, ApiError> {
    let keys = state.api_key_service.list().await?;
    Ok(Json(keys.into_iter().map(ApiKeyResponse::from).collect()))
}

pub async fn list_user_api_keys(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Vec<ApiKeyResponse>>, ApiError> {
    let keys = state.api_key_service.list_by_user(id).await?;
    Ok(Json(keys.into_iter().map(ApiKeyResponse::from).collect()))
}

pub async fn revoke_api_key(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiKeyResponse>, ApiError> {
    let key = state.api_key_service.revoke(id).await?;
    Ok(Json(ApiKeyResponse::from(key)))
}

pub async fn delete_api_key(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    state.api_key_service.delete(id).await?;
    Ok(StatusCode::NO_CONTENT)
}
