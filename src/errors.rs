use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};

use serde::Serialize;

use std::net::AddrParseError;
use thiserror::Error;
#[derive(Error, Debug)]
pub enum ReleasrError {
    #[error("io error")]
    IoError(#[from] std::io::Error),
    #[error("sqlite error")]
    SqliteError(#[from] rusqlite::Error),
    #[error("Actix error")]
    ActixError(#[from] actix_web::Error),
    #[error("Address error")]
    AddrParseError(#[from] AddrParseError),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
    #[error("NotFound error: {0}")]
    NotFound(String),
    #[error("Forbidden error")]
    Forbidden,
    #[error("Unknown error")]
    Unknown,
}

#[derive(Serialize)]
struct ErrorResponse {
    code: u16,
    error: String,
    message: String,
}

impl ResponseError for ReleasrError {
    fn status_code(&self) -> StatusCode {
        match *self {
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::Forbidden => StatusCode::FORBIDDEN,
            Self::Unknown => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let error_response = ErrorResponse {
            code: status_code.as_u16(),
            message: self.to_string(),
            error: "".to_string(),
        };
        HttpResponse::build(status_code).json(error_response)
    }
}
