use actix_web::{HttpResponse, ResponseError};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Not Found: {0}")]
    NotFound(String),
    #[error("Bad Request: {0}")]
    BadRequest(String),
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    #[error("Internal Error: {0}")]
    InternalError(String),
    #[error("Database Error: {0}")]
    DatabaseError(String),
    #[error("MQTT Error: {0}")]
    MqttError(String),
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::NotFound(ref msg) => HttpResponse::NotFound().json(json!({ "error": msg })),
            AppError::BadRequest(ref msg) => HttpResponse::BadRequest().json(json!({ "error": msg })),
            AppError::Unauthorized(ref msg) => HttpResponse::Unauthorized().json(json!({ "error": msg })),
            AppError::InternalError(ref msg) => HttpResponse::InternalServerError().json(json!({ "error": msg })),
            AppError::DatabaseError(ref msg) => HttpResponse::InternalServerError().json(json!({ "error": msg })),
            AppError::MqttError(ref msg) => HttpResponse::ServiceUnavailable().json(json!({ "error": msg })),
        }
    }
}

impl From<mongodb::error::Error> for AppError {
    fn from(err: mongodb::error::Error) -> Self {
        AppError::DatabaseError(err.to_string())
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::InternalError(err.to_string())
    }
}
