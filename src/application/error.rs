use thiserror::Error;

use crate::infrastructure::persistence::error::PersistenceError;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("not found")]
    NotFound,
    #[error("database error: {0}")]
    Db(#[from] PersistenceError),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}
