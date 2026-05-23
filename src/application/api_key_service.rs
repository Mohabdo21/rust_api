use std::sync::Arc;

use uuid::Uuid;

use crate::{
    application::{
        error::AppError,
        ports::{ApiKeyRepository, UserRepository},
    },
    domain::{
        api_key_secret::{generate_api_key_value, hash_api_key_value},
        models::ApiKey,
    },
};

pub struct CreatedApiKey {
    pub api_key: ApiKey,
    pub key_value: String,
}

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

    pub async fn create(
        &self,
        user_id: Uuid,
        label: Option<String>,
    ) -> Result<CreatedApiKey, AppError> {
        let exists = self.user_repo.find_by_id(user_id).await?;
        if exists.is_none() {
            return Err(AppError::NotFound);
        }

        let key_value = generate_api_key_value();
        let key_hash = hash_api_key_value(&key_value);
        let api_key = self
            .api_key_repo
            .create(Uuid::now_v7(), user_id, key_hash, label)
            .await?;

        Ok(CreatedApiKey { api_key, key_value })
    }

    pub async fn list(&self) -> Result<Vec<ApiKey>, AppError> {
        Ok(self.api_key_repo.list().await?)
    }

    pub async fn list_by_user(&self, user_id: Uuid) -> Result<Vec<ApiKey>, AppError> {
        Ok(self.api_key_repo.list_by_user(user_id).await?)
    }

    pub async fn revoke(&self, id: Uuid) -> Result<ApiKey, AppError> {
        self.api_key_repo
            .set_revoked(id, true)
            .await?
            .ok_or(AppError::NotFound)
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        let affected = self.api_key_repo.delete_by_id(id).await?;
        if affected == 0 {
            return Err(AppError::NotFound);
        }

        Ok(())
    }
}
