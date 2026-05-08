use async_trait::async_trait;
use sea_orm::DbErr;
use uuid::Uuid;

use crate::domain::models::{ApiKey, User};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, id: Uuid, name: String, email: String) -> Result<User, DbErr>;
    async fn list(&self) -> Result<Vec<User>, DbErr>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, DbErr>;
}

#[async_trait]
pub trait ApiKeyRepository: Send + Sync {
    async fn create(
        &self,
        id: Uuid,
        user_id: Uuid,
        key_value: String,
        label: Option<String>,
    ) -> Result<ApiKey, DbErr>;
    async fn list(&self) -> Result<Vec<ApiKey>, DbErr>;
    async fn list_by_user(&self, user_id: Uuid) -> Result<Vec<ApiKey>, DbErr>;
    async fn set_revoked(&self, id: Uuid, revoked: bool) -> Result<Option<ApiKey>, DbErr>;
    async fn delete_by_id(&self, id: Uuid) -> Result<u64, DbErr>;
}
