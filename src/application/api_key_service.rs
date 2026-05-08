use std::sync::Arc;

use uuid::Uuid;

use crate::{
    application::{
        error::AppError,
        ports::{ApiKeyRepository, UserRepository},
    },
    domain::models::ApiKey,
};

#[derive(Clone)]
pub struct ApiKeyService {
    api_key_repo: Arc<dyn ApiKeyRepository>,
    user_repo: Arc<dyn UserRepository>,
}

impl ApiKeyService {
    pub fn new(
        api_key_repo: Arc<dyn ApiKeyRepository>,
        user_repo: Arc<dyn UserRepository>,
    ) -> Self {
        Self {
            api_key_repo,
            user_repo,
        }
    }

    pub async fn create(&self, user_id: i32, label: Option<String>) -> Result<ApiKey, AppError> {
        let exists = self.user_repo.find_by_id(user_id).await?;
        if exists.is_none() {
            return Err(AppError::NotFound);
        }

        let key_value = Uuid::new_v4().to_string();
        Ok(self.api_key_repo.create(user_id, key_value, label).await?)
    }

    pub async fn list(&self) -> Result<Vec<ApiKey>, AppError> {
        Ok(self.api_key_repo.list().await?)
    }

    pub async fn list_by_user(&self, user_id: i32) -> Result<Vec<ApiKey>, AppError> {
        Ok(self.api_key_repo.list_by_user(user_id).await?)
    }

    pub async fn revoke(&self, id: i32) -> Result<ApiKey, AppError> {
        self.api_key_repo
            .set_revoked(id, true)
            .await?
            .ok_or(AppError::NotFound)
    }

    pub async fn delete(&self, id: i32) -> Result<(), AppError> {
        let affected = self.api_key_repo.delete_by_id(id).await?;
        if affected == 0 {
            return Err(AppError::NotFound);
        }

        Ok(())
    }
}
