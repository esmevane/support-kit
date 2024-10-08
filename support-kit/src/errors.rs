use std::io::Error;
use thiserror::Error;

#[derive(Debug, thiserror::Error)]
#[error("invalid service label: {0}")]
pub struct InvalidServiceLabelError(#[from] std::io::Error);

#[derive(Debug, thiserror::Error)]
pub enum ServiceControlError {
    #[error("Failed to initialize service control")]
    InitializationError(#[from] Error),

    #[error("invalid service label: {0}")]
    InvalidServiceLabelError(#[from] InvalidServiceLabelError),
}

#[derive(Debug, thiserror::Error)]
pub enum MissingDirError {
    #[error("missing home directory")]
    HomeDir,
    #[error("missing config directory")]
    ConfigDir,
}

#[derive(Debug, Error)]
pub enum SupportKitError {
    #[error("service control error: {0}")]
    ServiceControlError(#[from] ServiceControlError),

    #[error("problem finding directory: {0}")]
    MissingDirError(#[from] MissingDirError),

    #[error("problem building config: {0}")]
    ConfigBuildError(#[from] figment::Error),
}
