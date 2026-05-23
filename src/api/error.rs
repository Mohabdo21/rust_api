use axum::{
    Json,
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use tracing::error;

use crate::application::error::AppError;

pub enum ApiError {
    App(AppError),
    Request {
        status: StatusCode,
        error: &'static str,
        message: String,
    },
}

#[derive(Serialize)]
struct ErrorResponse {
    error: &'static str,
    message: String,
}

impl From<AppError> for ApiError {
    fn from(value: AppError) -> Self {
        Self::App(value)
    }
}

impl ApiError {
    pub fn from_json_rejection(rejection: JsonRejection) -> Self {
        match rejection {
            JsonRejection::JsonSyntaxError(_) => Self::request(
                StatusCode::BAD_REQUEST,
                "invalid_request",
                "request body contains invalid JSON".to_string(),
            ),
            JsonRejection::JsonDataError(_) => Self::request(
                StatusCode::BAD_REQUEST,
                "invalid_request",
                "request body is invalid".to_string(),
            ),
            JsonRejection::MissingJsonContentType(_) => Self::request(
                StatusCode::UNSUPPORTED_MEDIA_TYPE,
                "unsupported_media_type",
                "content-type must be application/json".to_string(),
            ),
            JsonRejection::BytesRejection(_) => Self::request(
                StatusCode::BAD_REQUEST,
                "invalid_request",
                "request body could not be read".to_string(),
            ),
            _ => Self::request(
                StatusCode::BAD_REQUEST,
                "invalid_request",
                "request body is invalid".to_string(),
            ),
        }
    }

    fn request(status: StatusCode, error: &'static str, message: String) -> Self {
        Self::Request {
            status,
            error,
            message,
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            Self::App(app_error) => match app_error {
                AppError::NotFound => {
                    error_response(StatusCode::NOT_FOUND, "not_found", "not found".to_string())
                }
                AppError::Validation(message) => error_response(
                    StatusCode::UNPROCESSABLE_ENTITY,
                    "validation_error",
                    message,
                ),
                AppError::Conflict(message) => {
                    error_response(StatusCode::CONFLICT, "conflict", message)
                }
                AppError::Db(err) => {
                    error!(error = %err, "database request failed");
                    error_response(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "internal_server_error",
                        "internal server error".to_string(),
                    )
                }
                AppError::Io(err) => {
                    error!(error = %err, "io request failed");
                    error_response(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "internal_server_error",
                        "internal server error".to_string(),
                    )
                }
            },
            Self::Request {
                status,
                error,
                message,
            } => error_response(status, error, message),
        }
    }
}

fn error_response(status: StatusCode, error: &'static str, message: String) -> Response {
    (status, Json(ErrorResponse { error, message })).into_response()
}
