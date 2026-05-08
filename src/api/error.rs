use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::application::error::AppError;

pub struct ApiError(pub AppError);

impl From<AppError> for ApiError {
    fn from(value: AppError) -> Self {
        Self(value)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self.0 {
            AppError::NotFound => (StatusCode::NOT_FOUND, "not found").into_response(),
            AppError::Db(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("database error: {err}"),
            )
                .into_response(),
        }
    }
}
