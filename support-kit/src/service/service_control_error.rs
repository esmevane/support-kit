use std::io::Error;

#[derive(Debug, thiserror::Error)]
pub enum ServiceControlError {
    #[error("Failed to initialize service control")]
    InitializationError(#[from] Error),
}
