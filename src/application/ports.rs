use async_trait::async_trait;
use sea_orm::DbErr;

use crate::domain::models::{ApiKey, User};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, name: String, email: String) -> Result<User, DbErr>;
    async fn list(&self) -> Result<Vec<User>, DbErr>;
    async fn find_by_id(&self, id: i32) -> Result<Option<User>, DbErr>;
}

#[async_trait]
pub trait ApiKeyRepository: Send + Sync {
    async fn create(
        &self,
        user_id: i32,
        key_value: String,
        label: Option<String>,
    ) -> Result<ApiKey, DbErr>;
    async fn list(&self) -> Result<Vec<ApiKey>, DbErr>;
    async fn list_by_user(&self, user_id: i32) -> Result<Vec<ApiKey>, DbErr>;
    async fn set_revoked(&self, id: i32, revoked: bool) -> Result<Option<ApiKey>, DbErr>;
    async fn delete_by_id(&self, id: i32) -> Result<u64, DbErr>;
}
