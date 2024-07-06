#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Request error: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("Config error: {0}")]
    ConfigError(#[from] config::ConfigError),
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Unable to initialize tcp listener: {0}")]
    ListenerInitFailure(std::io::Error),
    #[error("Unable to parse selected option: {0}")]
    CliOptionSelectError(#[from] strum::ParseError),
    #[error("Support error: {0}")]
    SupportError(#[from] service_kit_support::errors::Error),
}
