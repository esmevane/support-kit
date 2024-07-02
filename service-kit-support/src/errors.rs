#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Storage file error: {0}")]
    StorageFileError(#[from] sqlx::Error),
    #[error("Storage error: {0}")]
    StorageError(#[from] crate::storage::StorageError),
    #[error("Storage not configured, unable to initialize storage collection")]
    StorageNotConfiguredError,
}
