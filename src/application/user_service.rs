use std::sync::Arc;
use uuid::Uuid;

use crate::{
    application::{error::AppError, ports::UserRepository},
    domain::models::User,
};

#[derive(Clone)]
pub struct UserService {
    repo: Arc<dyn UserRepository>,
}

impl UserService {
    pub fn new(repo: Arc<dyn UserRepository>) -> Self {
        Self { repo }
    }

    pub async fn create(&self, name: String, email: String) -> Result<User, AppError> {
        Ok(self.repo.create(Uuid::now_v7(), name, email).await?)
    }

    pub async fn list(&self) -> Result<Vec<User>, AppError> {
        Ok(self.repo.list().await?)
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        let affected = self.repo.delete_by_id(id).await?;
        if affected == 0 {
            return Err(AppError::NotFound);
        }
        Ok(())
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<User, AppError> {
        self.repo.find_by_id(id).await?.ok_or(AppError::NotFound)
    }
}
