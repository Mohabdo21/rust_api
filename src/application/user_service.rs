use std::sync::Arc;

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
        Ok(self.repo.create(name, email).await?)
    }

    pub async fn list(&self) -> Result<Vec<User>, AppError> {
        Ok(self.repo.list().await?)
    }

    pub async fn get_by_id(&self, id: i32) -> Result<User, AppError> {
        self.repo.find_by_id(id).await?.ok_or(AppError::NotFound)
    }
}
