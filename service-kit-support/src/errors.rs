#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Config error: {0}")]
    ConfigError(#[from] config::ConfigError),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Storage error: {0}")]
    StorageError(#[from] crate::storage::StorageError),
    #[error("Storage file error: {0}")]
    StorageFileError(#[from] sqlx::Error),
    #[error("Storage not configured, unable to initialize storage collection")]
    StorageNotConfiguredError,
    #[error("Unable to get next terminal event")]
    TerminalEventError,
}
