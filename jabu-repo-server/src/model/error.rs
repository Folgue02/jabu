use axum::{response::IntoResponse, Json, http::StatusCode};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct ApiError {
    status_code: u16,
    description: String,
}

impl ApiError {
    pub fn new(status_code: StatusCode, description: impl Into<String>) -> Self {
        Self {
            status_code: status_code.as_u16(),
            description: description.into(),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::from_u16(self.status_code).unwrap(), Json(self)).into_response()
    }
}

impl From<sqlx::Error> for ApiError {
    fn from(value: sqlx::Error) -> Self {
        if cfg!(debug_assertions) {
            Self {
                status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                description: format!("DATABASE ERROR: \n{}", value),
            }
        } else {
            Self {
                status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                description: "Internal database error".to_string(),
            }
        }
    }
}
