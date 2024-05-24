use thiserror::Error;

#[derive(Error, Debug)]
pub enum FrontendError {
    #[error("Network error: {0}")]
    NetworkError(String),
    
    #[error("Failed to fetch data")]
    FetchError,
    
    #[error("Unexpected error: {0}")]
    UnexpectedError(String),
}
