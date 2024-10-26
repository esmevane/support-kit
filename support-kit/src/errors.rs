use std::{io::Error, net::AddrParseError};
use thiserror::Error;

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
}
