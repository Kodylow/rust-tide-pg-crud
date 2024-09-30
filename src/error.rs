use sqlx::Error as SqlxError;
use std::{io::Error as IoError, string::ParseError};
use tera::Error as TeraError;
use thiserror::Error;
use tide::StatusCode;
use uuid::Error as UuidError;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Parse error: {0}")]
    Parse(#[from] ParseError),

    #[error("Database error: {0}")]
    Database(#[from] SqlxError),

    #[error("Http error: {status} {message}")]
    Http {
        status: StatusCode,
        message: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    #[error("I/O error: {0}")]
    Io(#[from] IoError),

    #[error("Template rendering error: {0}")]
    Tera(#[from] TeraError),

    #[error("UUID error: {0}")]
    Uuid(#[from] UuidError),

    #[error("Environment variable not found: {0}")]
    EnvVar(String),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Internal server error: {0}")]
    InternalServerError(String),
}
