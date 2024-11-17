use std::{io::Error, net::AddrParseError};
use thiserror::Error;

/// The auth token failed to verify.
#[derive(thiserror::Error, Debug)]
#[error("Token verification failed: {0}")]
pub struct AuthTokenVerificationFailure(#[from] jsonwebtoken::errors::Error);

/// We couldn't make an auth token.
#[derive(thiserror::Error, Debug)]
#[error("Unable to generate auth token: {0}")]
pub struct AuthTokenGenerationFailure(#[from] jsonwebtoken::errors::Error);

/// Something went wrong with our session token.
#[derive(thiserror::Error, Debug)]
#[error(transparent)]
pub enum TokenError {
    /// An ID parse error means the ID in the token is not a valid uuid.
    InvalidUuid(#[from] uuid::Error),
    /// We couldn't verify the token.
    VerificationFailed(#[from] AuthTokenVerificationFailure),
    /// We couldn't make the token.
    TokenGenerationFailure(#[from] AuthTokenGenerationFailure),
}

#[derive(Debug, thiserror::Error)]
pub enum PasswordError {
    #[error("Failed to hash password: {0}")]
    HashError(argon2::password_hash::Error),
}

impl From<argon2::password_hash::Error> for PasswordError {
    fn from(err: argon2::password_hash::Error) -> Self {
        PasswordError::HashError(err)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum BoilerplateError {
    #[error("problem with template: {0}")]
    TemplateError(#[from] minijinja::Error),
    #[error("template persistence error: {0}")]
    IoError(#[from] std::io::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum ShellCommandError {
    #[error("unable to execute command: {0}")]
    ExecError(#[from] std::io::Error),
    #[error("malformed command, unable to parse: {0}")]
    MalformedError(String),
}

#[derive(Debug, thiserror::Error)]
#[error("network init error: {0}")]
pub struct NetworkInitError(#[from] AddrParseError);

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
pub enum SshError {
    #[error("connection error: {0}")]
    SshError(#[from] russh::Error),

    #[error("key error: {0}")]
    SshKeyError(#[from] russh::keys::Error),

    #[error("channel write error: {0}")]
    SshIoError(#[from] std::io::Error),

    #[error("authentication failed")]
    AuthenticationFailed,
    #[error("invalid path: {0}")]
    InvalidPath(String),
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

    #[error("problem initializing network: {0}")]
    NetworkInitError(#[from] NetworkInitError),

    #[error("ssh error: {0}")]
    SshError(#[from] SshError),

    #[error("serde error: {0}")]
    SerdeError(#[from] serde_json::Error),

    #[error("ops process error: {0}")]
    OpsProcessError(#[from] ShellCommandError),

    #[error("boilerplate error: {0}")]
    BoilerplateError(#[from] BoilerplateError),

    #[error("token error: {0}")]
    TokenError(#[from] TokenError),

    #[error("password error: {0}")]
    PasswordError(#[from] PasswordError),
}
