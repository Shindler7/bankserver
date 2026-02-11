//! Слой ошибок для API.

use crate::errors::BankError;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BankApiError {
    #[error("{0}")]
    Domain(#[from] BankError),
}

impl ResponseError for BankApiError {
    fn error_response(&self) -> HttpResponse {
        let (status, details) = match self {
            BankApiError::Domain(err) => match err {
                BankError::Validation(_)
                | BankError::InsufficientFunds
                | BankError::Database(_) => (StatusCode::BAD_REQUEST, err.to_string()),
                BankError::NotFound(_) => (StatusCode::NOT_FOUND, err.to_string()),
                BankError::Unauthorized => (StatusCode::UNAUTHORIZED, err.to_string()),
            },
        };

        HttpResponse::build(status).json(serde_json::json!({
            "error": status.as_u16(),
            "detail": details
        }))
    }
}
