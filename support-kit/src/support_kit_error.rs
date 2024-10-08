use thiserror::Error;

use crate::ServiceControlError;

#[derive(Debug, Error)]
pub enum SupportKitError {
    #[error("service control error: {0}")]
    ServiceControlError(#[from] ServiceControlError),
}
