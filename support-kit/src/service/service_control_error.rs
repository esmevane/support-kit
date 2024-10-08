use std::io::Error;

use super::service_name::InvalidServiceLabelError;

#[derive(Debug, thiserror::Error)]
pub enum ServiceControlError {
    #[error("Failed to initialize service control")]
    InitializationError(#[from] Error),

    #[error("invalid service label: {0}")]
    InvalidServiceLabelError(#[from] InvalidServiceLabelError),
}
