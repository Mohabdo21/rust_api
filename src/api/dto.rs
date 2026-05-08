use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::models::{ApiKey, User};

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateApiKeyRequest {
    pub user_id: Uuid,
    pub label: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ApiKeyResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub key_value: String,
    pub label: Option<String>,
    pub revoked: bool,
}

impl From<User> for UserResponse {
    fn from(value: User) -> Self {
        Self {
            id: value.id,
            name: value.name,
            email: value.email,
        }
    }
}

impl From<ApiKey> for ApiKeyResponse {
    fn from(value: ApiKey) -> Self {
        Self {
            id: value.id,
            user_id: value.user_id,
            key_value: value.key_value,
            label: value.label,
            revoked: value.revoked,
        }
    }
}
