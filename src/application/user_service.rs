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
        let name = normalize_name(name)?;
        let email = normalize_email(email)?;

        match self.repo.create(Uuid::now_v7(), name, email).await {
            Ok(user) => Ok(user),
            Err(err) if err.is_unique_violation() => {
                Err(AppError::Conflict("email already exists".to_string()))
            }
            Err(err) => Err(AppError::Db(err)),
        }
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

fn normalize_name(name: String) -> Result<String, AppError> {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        return Err(AppError::Validation("name must not be empty".to_string()));
    }

    Ok(trimmed.to_string())
}

fn normalize_email(email: String) -> Result<String, AppError> {
    let trimmed = email.trim();
    if !is_valid_email(trimmed) {
        return Err(AppError::Validation(
            "email must be a valid address".to_string(),
        ));
    }

    Ok(trimmed.to_string())
}

fn is_valid_email(email: &str) -> bool {
    if email.is_empty() || email.chars().any(char::is_whitespace) {
        return false;
    }

    if email.matches('@').count() != 1 {
        return false;
    }

    let Some((local_part, domain_part)) = email.split_once('@') else {
        return false;
    };

    !local_part.is_empty()
        && !domain_part.is_empty()
        && domain_part.contains('.')
        && !domain_part.starts_with('.')
        && !domain_part.ends_with('.')
}
