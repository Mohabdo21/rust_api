use thiserror::Error;

#[derive(Debug, Error)]
pub enum PersistenceError {
    #[error("database query error: {0}")]
    Query(#[from] diesel::result::Error),
    #[error("database pool error: {0}")]
    Pool(#[from] diesel::r2d2::PoolError),
    #[error("database migration error: {0}")]
    Migration(String),
    #[error("background task failed: {0}")]
    TaskJoin(#[from] tokio::task::JoinError),
    #[error("invalid uuid in {field}: {source}")]
    InvalidUuid {
        field: String,
        #[source]
        source: uuid::Error,
    },
    #[error("invalid database url: {0}")]
    InvalidDatabaseUrl(String),
}
