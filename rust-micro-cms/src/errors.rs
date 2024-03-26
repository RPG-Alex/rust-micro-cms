use axum::error_handling::HandleError;
use serde::Serialize;
use sqlx::error::Error as SQLxError;
use std::fmt;


#[derive(Debug, Serialize)]
pub enum RustMicroCmsError {
    DBError(String),
    AxumError(String),
    NotFound(String),
    InvalidINput(String),
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    error_message: String,
}

impl std::error::Error for RustMicroCmsError {}

impl RustMicroCmsError {
    fn error_response(&self) -> String {
        match self {
            RustMicroCmsError::DBError(msg) => {
                // TODO: IMplement error handling to log file
            }
        }
    }
}


