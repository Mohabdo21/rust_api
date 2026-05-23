use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    domain::models::{ApiKey, User},
    infrastructure::persistence::error::PersistenceError,
};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, id: Uuid, name: String, email: String)
    -> Result<User, PersistenceError>;
    async fn list(&self) -> Result<Vec<User>, PersistenceError>;
    async fn delete_by_id(&self, id: Uuid) -> Result<u64, PersistenceError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, PersistenceError>;
}

#[async_trait]
pub trait ApiKeyRepository: Send + Sync {
    async fn create(
        &self,
        id: Uuid,
        user_id: Uuid,
        key_hash: String,
        label: Option<String>,
    ) -> Result<ApiKey, PersistenceError>;
    async fn list(&self) -> Result<Vec<ApiKey>, PersistenceError>;
    async fn list_by_user(&self, user_id: Uuid) -> Result<Vec<ApiKey>, PersistenceError>;
    async fn set_revoked(
        &self,
        id: Uuid,
        revoked: bool,
    ) -> Result<Option<ApiKey>, PersistenceError>;
    async fn delete_by_id(&self, id: Uuid) -> Result<u64, PersistenceError>;
}
